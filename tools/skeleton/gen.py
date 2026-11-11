"""Generate Rust skeletons for SUMO C++ directories from the clang AST.

For every class/struct/enum defined in a header of the target directories it emits a Rust file with
the type (fields mapped to Rust types), an `impl` with every method signature, and a `todo!()` body
preceded by the original C++ body as a comment. Agents then replace each todo!() with a translation.

Run inside WSL (needs python3-clang and the C++ std headers):
    python3 gen.py microsim netload ... [--out ../../crates/sumo_core/src] [--jobs 32]

Mapping rules (keep in sync with AGENTS.md):
  T* / const T* (class)   -> Ptr<T>              std::string          -> String (params: &str)
  T& / const T&           -> &mut T / &T          std::vector<T>       -> Vec<T>
  std::map<K,V>           -> BTreeMap<K,V>        std::set<T>          -> BTreeSet<T>
  std::unordered_map      -> HashMap (determinism)  std::pair<A,B>     -> (A, B)
  std::list / std::deque  -> VecDeque             unknown types        -> Unmapped<"..."> placeholder alias
  base class B            -> field `base: B` + Deref/DerefMut to it
  virtual methods         -> plain methods on the struct; overrides noted with `// virtual (overrides Base)`
"""
import os, re, sys, json, multiprocessing as mp
import clang.cindex as ci

SRC = "/mnt/c/Users/karti/sumo-src/src"
HERE = os.path.dirname(os.path.abspath(__file__))
ARGS = ["-x", "c++", "-std=c++17", "-I", SRC, "-I", os.path.join(HERE, "fake_include")]
K = ci.CursorKind

PRIM = {
    "double": "f64", "float": "f32", "int": "i32", "long": "i64", "long long": "i64", "long int": "i64",
    "long long int": "i64", "unsigned int": "u32", "unsigned": "u32", "unsigned long": "u64",
    "unsigned long long": "u64", "unsigned long int": "u64", "unsigned long long int": "u64",
    "short": "i16", "unsigned short": "u16", "char": "i8", "unsigned char": "u8", "signed char": "i8",
    "bool": "bool", "void": "()", "size_t": "usize", "std::size_t": "usize", "uint8_t": "u8",
    "uint16_t": "u16", "uint32_t": "u32", "uint64_t": "u64", "int8_t": "i8", "int16_t": "i16",
    "int32_t": "i32", "int64_t": "i64", "SUMOTime": "SUMOTime", "SVCPermissions": "SVCPermissions",
    "std::string": "String", "string": "String",
}
CONTAINERS = {
    "std::vector": "Vec", "std::map": "BTreeMap", "std::set": "BTreeSet", "std::multimap": "BTreeMap",
    "std::list": "VecDeque", "std::deque": "VecDeque", "std::unordered_map": "HashMap",
    "std::unordered_set": "HashSet", "std::queue": "VecDeque", "std::priority_queue": "BinaryHeap",
    "std::shared_ptr": "Rc", "std::unique_ptr": "Box", "std::array": "Vec", "std::atomic": "",
    "std::function": "Box<dyn Fn>",
}
RUST_KW = {"type", "match", "move", "ref", "fn", "loop", "mod", "use", "where", "impl", "trait", "self",
           "super", "crate", "box", "in", "as", "dyn", "gen", "async", "await", "yield", "override",
           "final", "priv", "abstract", "macro", "unsafe", "pub", "mut", "let", "struct", "enum", "Self"}


def split_top(s, sep=","):
    out, depth, cur = [], 0, ""
    for ch in s:
        if ch in "<(":
            depth += 1
        elif ch in ">)":
            depth -= 1
        if ch == sep and depth == 0:
            out.append(cur.strip()); cur = ""
        else:
            cur += ch
    if cur.strip():
        out.append(cur.strip())
    return out


class TypeMapper:
    def __init__(self, known):
        self.known = known  # {C++ name, qualified "Outer::Inner" or bare: Rust type name}
        self.unmapped = set()

    def lookup(self, cpp):
        parts = [p for p in cpp.split("::") if p not in ("", "std", "libsumo", "LIBSUMO_NAMESPACE", "PHEMlightdllV5", "RoutingKit")]
        for i in range(len(parts)):  # longest qualified suffix first: A::B::C, B::C, C
            key = "::".join(parts[i:])
            if key in self.known:
                return self.known[key]
        return None

    def map(self, t, param=False, ret=False):
        t = re.sub(r"\b(struct|class|enum|typename)\s+", "", t).strip()
        t = t.replace("std::__cxx11::", "std::").replace("std::__1::", "std::")
        if t.endswith("&&"):
            return self.map(t[:-2].strip(), param, ret)
        if t.endswith("&"):
            inner = t[:-1].strip()
            const = inner.startswith("const ") or inner.endswith(" const")
            inner = re.sub(r"^const\s+|\s+const$", "", inner)
            if inner in ("std::string", "string"):
                return "&str" if (const and (param or ret)) else "&mut String"
            m = self.map(inner)
            return ("&" if const else "&mut ") + m
        t = re.sub(r"\s*const$", "", t)
        if t.endswith("*"):
            inner = re.sub(r"^const\s+|\s+const$", "", t[:-1].strip())
            if inner in ("char",):
                return "&str"
            if inner == "void":
                return "Ptr<()>"
            return f"Ptr<{self.map(inner)}>"
        t = re.sub(r"^const\s+", "", t)
        if param and t in ("std::string", "string"):
            return "String"
        if t in PRIM:
            return PRIM[t]
        m = re.match(r"^([\w:]+)\s*<(.*)>$", t)
        if m:
            name, args = m.group(1), split_top(m.group(2))
            if name == "std::pair":
                return "(" + ", ".join(self.map(a) for a in args[:2]) + ")"
            if name in ("std::set", "std::map", "std::multimap") and len(args) > (1 if name == "std::set" else 2):
                args = args[: (1 if name == "std::set" else 2)]  # custom comparator: noted by caller
            if name == "std::array":
                return f"[{self.map(args[0])}; {args[1]}]"
            if name == "std::atomic":
                return self.map(args[0])
            if name == "std::function":
                return "Box<dyn Fn()>"
            if name in CONTAINERS:
                return f"{CONTAINERS[name]}<{', '.join(self.map(a) for a in args)}>"
            base = self.lookup(name)
            if base:
                return f"{base}<{', '.join(self.map(a) for a in args)}>"
        found = self.lookup(t)
        if found:
            return found
        bare = t.split("::")[-1]
        if re.fullmatch(r"[A-Z]\w*", bare) and len(bare) <= 2:  # template parameter like T, E, V
            return bare
        self.unmapped.add(t)
        return f"Unmapped /* {t} */"


def snake(name):
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name)
    s = re.sub(r"(?<=[A-Z])([A-Z][a-z])", r"_\1", s)
    s = re.sub(r"\W", "_", s).lower().strip("_")
    return s + "_" if s in RUST_KW else s


def ident(name):
    return name + "_" if name in RUST_KW else name


def src_text(cursor):
    ext = cursor.extent
    if not ext.start.file:
        return ""
    with open(ext.start.file.name, encoding="utf8", errors="replace") as f:
        data = f.read().encode("utf8", errors="replace")
    return data[ext.start.offset:ext.end.offset].decode("utf8", errors="replace")


def comment_block(text, indent):
    return "".join(f"{indent}// {l.rstrip()}\n" if l.strip() else f"{indent}//\n" for l in text.splitlines())


def parse(path):
    idx = ci.Index.create()
    return idx.parse(path, args=ARGS, options=ci.TranslationUnit.PARSE_INCOMPLETE)


def in_dirs(cursor, dirs):
    f = cursor.location.file
    if not f:
        return None
    rel = os.path.relpath(f.name, SRC).replace("\\", "/")
    return rel if any(rel.startswith(d + "/") and rel.count("/") == d.count("/") + 1 for d in dirs) else None


def collect(args):
    """Parse one .cpp/.h file; return {usr: decl-info} for types defined in target dirs + method bodies."""
    path, dirs = args
    try:
        tu = parse(path)
    except Exception as e:  # noqa: BLE001
        return {}, {}, [f"{path}: {e}"]
    types, bodies = {}, {}

    def visit(c, ns=""):
        for ch in c.get_children():
            rel = in_dirs(ch, dirs)
            if ch.kind == K.NAMESPACE:
                visit(ch, ns + ch.spelling + "::")
            elif ch.kind in (K.CLASS_DECL, K.STRUCT_DECL, K.CLASS_TEMPLATE) and ch.is_definition() and rel:
                types.setdefault(ch.get_usr(), describe_class(ch, rel, ns))
            elif ch.kind == K.ENUM_DECL and ch.is_definition() and rel and ch.spelling:
                types.setdefault(ch.get_usr(), describe_enum(ch, rel, ns))
            elif ch.kind in (K.CXX_METHOD, K.CONSTRUCTOR, K.DESTRUCTOR, K.FUNCTION_DECL, K.FUNCTION_TEMPLATE) \
                    and ch.is_definition():
                frel = in_dirs(ch, dirs)
                if frel:
                    sem = ch.semantic_parent
                    key = (sem.get_usr() if sem and sem.kind != K.TRANSLATION_UNIT and sem.kind != K.NAMESPACE else "",
                           ch.spelling, ch.type.spelling)
                    bodies.setdefault(json.dumps(key), src_text(ch))
                    if ch.kind in (K.FUNCTION_DECL, K.FUNCTION_TEMPLATE) and (not sem or sem.kind in (K.TRANSLATION_UNIT, K.NAMESPACE)):
                        types.setdefault("fn:" + ch.get_usr(), describe_function(ch, frel, ns))
            elif ch.kind in (K.TYPEDEF_DECL, K.TYPE_ALIAS_DECL) and rel:
                parent = ch.semantic_parent.spelling if ch.semantic_parent.kind in (K.CLASS_DECL, K.STRUCT_DECL, K.CLASS_TEMPLATE) else ""
                types.setdefault("td:" + ch.get_usr(), {"kind": "typedef", "name": ch.spelling, "file": rel, "ns": ns,
                                                         "target": ch.underlying_typedef_type.spelling, "parent": parent})
            if ch.kind in (K.CLASS_DECL, K.STRUCT_DECL, K.CLASS_TEMPLATE) and rel:
                visit(ch, ns)  # nested classes / inline method definitions
    visit(tu.cursor)
    return types, bodies, []


def describe_function(c, rel, ns):
    return {"kind": "fn", "name": c.spelling, "file": rel, "ns": ns, "usr": c.get_usr(),
            "ret": c.result_type.spelling, "type": c.type.spelling,
            "params": [(a.spelling, a.type.spelling) for a in c.get_arguments()], "static": True}


def describe_enum(c, rel, ns):
    vals = [(e.spelling, e.enum_value) for e in c.get_children() if e.kind == K.ENUM_CONSTANT_DECL]
    parent = c.semantic_parent.spelling if c.semantic_parent.kind in (K.CLASS_DECL, K.STRUCT_DECL) else ""
    return {"kind": "enum", "name": c.spelling, "file": rel, "ns": ns, "values": vals, "parent": parent,
            "doc": c.brief_comment or ""}


def describe_class(c, rel, ns):
    info = {"kind": "class", "name": c.spelling, "file": rel, "ns": ns, "usr": c.get_usr(), "bases": [],
            "fields": [], "statics": [], "methods": [], "tparams": [], "doc": c.brief_comment or "",
            "parent": c.semantic_parent.spelling if c.semantic_parent.kind in (K.CLASS_DECL, K.STRUCT_DECL, K.CLASS_TEMPLATE) else ""}
    for ch in c.get_children():
        if ch.kind == K.CXX_BASE_SPECIFIER:
            info["bases"].append(ch.type.spelling)
        elif ch.kind == K.TEMPLATE_TYPE_PARAMETER:
            info["tparams"].append(ch.spelling)
        elif ch.kind == K.FIELD_DECL:
            info["fields"].append((ch.spelling, ch.type.spelling, ch.brief_comment or ""))
        elif ch.kind == K.VAR_DECL:
            info["statics"].append((ch.spelling, ch.type.spelling))
        elif ch.kind in (K.CXX_METHOD, K.CONSTRUCTOR, K.DESTRUCTOR, K.FUNCTION_TEMPLATE):
            info["methods"].append({
                "name": ch.spelling, "kind": ch.kind.name, "type": ch.type.spelling,
                "ret": ch.result_type.spelling, "params": [(a.spelling, a.type.spelling) for a in ch.get_arguments()],
                "static": ch.is_static_method(), "const": ch.is_const_method(), "virtual": ch.is_virtual_method(),
                "pure": ch.is_pure_virtual_method(), "access": ch.access_specifier.name, "doc": ch.brief_comment or ""})
    return info


OPS = {"operator==": "eq_op", "operator!=": "ne_op", "operator<": "lt_op", "operator>": "gt_op",
       "operator<=": "le_op", "operator>=": "ge_op", "operator()": "call_op", "operator[]": "index_op",
       "operator=": "assign_op", "operator+": "add_op", "operator-": "sub_op", "operator*": "mul_op",
       "operator/": "div_op", "operator+=": "add_assign_op", "operator-=": "sub_assign_op",
       "operator*=": "mul_assign_op", "operator/=": "div_assign_op", "operator<<": "shl_op",
       "operator bool": "as_bool_op", "operator!": "not_op"}


def emit_method(m, cls, tm, bodies, cls_usr, used_names, indent="    "):
    name = m["name"]
    if m["kind"] == "DESTRUCTOR":
        rust = "drop_"
    elif m["kind"] == "CONSTRUCTOR":
        rust = "new"
    else:
        rust = OPS.get(name, snake(name) if not name.startswith("operator") else "op_" + snake(name[8:]) or "op")
    base, n = rust, 2
    while rust in used_names:
        rust = f"{base}_{n}"; n += 1
    used_names.add(rust)
    params = []
    if m["kind"] == "CONSTRUCTOR" or m["static"]:
        pass
    else:
        params.append("&self" if m["const"] else "&mut self")
    for i, (pn, pt) in enumerate(m["params"]):
        params.append(f"{ident(pn) if pn else f'arg{i}'}: {tm.map(pt, param=True)}")
    ret = "Self" if m["kind"] == "CONSTRUCTOR" else ("" if m["ret"] in ("void", "") else tm.map(m["ret"], ret=True))
    if ret.startswith("&") and "self" not in (params[0] if params else ""):
        ret = ret.replace("&", "&'static ", 1)
    sig = f"pub fn {rust}{f'<{', '.join(cls_tparams(cls))}>' if False else ''}({', '.join(params)})" + (f" -> {ret}" if ret else "")
    key = json.dumps([cls_usr, name, m["type"]])
    body = bodies.get(key, "")
    out = ""
    flags = [f for f in ("static" if m["static"] else "", "const" if m["const"] else "",
                         "virtual" if m["virtual"] else "", "pure virtual" if m["pure"] else "",
                         m["access"].lower() if m["access"] != "PUBLIC" else "") if f]
    out += f"{indent}/// C++: `{m['ret']} {cls['name']}::{name}{m['type'][len(m['ret']):].strip() if m['type'].startswith(m['ret']) else m['type']}`"
    out += f" [{', '.join(flags)}]\n" if flags else "\n"
    if m["doc"]:
        out += f"{indent}/// {m['doc']}\n"
    if body:
        out += comment_block(body, indent)
    elif m["pure"]:
        out += f"{indent}// (pure virtual: no C++ body; subclasses implement it)\n"
    else:
        out += f"{indent}// (no C++ body found: implicit/defaulted or defined elsewhere)\n"
    out += f"{indent}#[allow(unused_variables)]\n{indent}{sig} {{\n{indent}    todo!(\"port {cls['name']}::{name}\")\n{indent}}}\n\n"
    return out


def cls_tparams(cls):
    return cls.get("tparams", [])


def emit_class(cls, tm, bodies):
    tp = cls_tparams(cls)
    gen = f"<{', '.join(tp)}>" if tp else ""
    out = ""
    if cls["doc"]:
        out += f"/// {cls['doc']}\n"
    out += f"/// C++ class `{cls['ns']}{(cls['parent'] + '::') if cls['parent'] else ''}{cls['name']}` ({cls['file']})\n"
    out += f"pub struct {cls['rust']}{gen} {{\n"
    bases = []
    for i, b in enumerate(cls["bases"]):
        bt = tm.map(b)
        fname = "base" if i == 0 else f"base{i + 1}"
        bases.append((fname, bt, b))
        out += f"    /// C++ base class `{b}`\n    pub {fname}: {bt},\n"
    for fn, ft, doc in cls["fields"]:
        note = f" // C++: {ft}" if ("std::set<" in ft or "std::map<" in ft) and ft.count(",") >= (1 if "set<" in ft else 2) else ""
        if "unordered" in ft:
            note = f" // determinism: C++ {ft}"
        if doc:
            out += f"    /// {doc}\n"
        out += f"    pub {ident(fn)}: {tm.map(ft)},{note}\n"
    if tp:
        out += f"    _p: std::marker::PhantomData<({', '.join(tp)},)>,\n"
    out += "}\n\n"
    if bases:
        fname, bt, _ = bases[0]
        out += (f"impl{gen} std::ops::Deref for {cls['rust']}{gen} {{\n    type Target = {bt};\n"
                f"    fn deref(&self) -> &{bt} {{\n        &self.{fname}\n    }}\n}}\n"
                f"impl{gen} std::ops::DerefMut for {cls['rust']}{gen} {{\n"
                f"    fn deref_mut(&mut self) -> &mut {bt} {{\n        &mut self.{fname}\n    }}\n}}\n\n")
    if cls["statics"]:
        out += f"// C++ static members of {cls['name']} (port as module-level statics, see AGENTS.md):\n"
        for sn, st in cls["statics"]:
            out += f"//   static {st} {cls['name']}::{sn};\n"
        out += "\n"
    out += f"impl{gen} {cls['rust']}{gen} {{\n"
    used = set()
    for m in cls["methods"]:
        out += emit_method(m, cls, tm, bodies, cls["usr"], used)
    out += "}\n\n"
    return out


def emit_enum(e):
    out = f"/// C++ enum `{e['ns']}{(e['parent'] + '::') if e['parent'] else ''}{e['name']}`\n"
    out += "#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n#[repr(i64)]\n"
    out += f"pub enum {e['rust']} {{\n"
    seen = set()
    dupes = []
    for n, v in e["values"]:
        if v in seen:
            dupes.append((n, v))
            continue
        seen.add(v)
        out += f"    {n} = {v},\n"
    out += "}\n"
    if dupes:
        out += f"impl {e['rust']} {{\n" + "".join(f"    pub const {n}: Self = Self::{next(m for m, w in e['values'] if w == v)};\n" for n, v in dupes) + "}\n"
    return out + "\n"


def emit_fn(f, tm, bodies):
    params = ", ".join(f"{ident(pn) if pn else f'arg{i}'}: {tm.map(pt, param=True)}" for i, (pn, pt) in enumerate(f["params"]))
    ret = "" if f["ret"] == "void" else " -> " + tm.map(f["ret"], ret=True).replace("&", "&'static ", 1)
    key = json.dumps(["", f["name"], f["type"]])
    body = bodies.get(key, "")
    out = f"/// C++: `{f['type']} {f['ns']}{f['name']}`\n" + (comment_block(body, "") if body else "")
    return out + f"#[allow(unused_variables)]\npub fn {snake(f['name'])}({params}){ret} {{\n    todo!(\"port {f['name']}\")\n}}\n\n"


def main():
    argv = sys.argv[1:]
    out_dir = os.path.join(HERE, "..", "..", "crates", "sumo_core", "src")
    jobs = 32
    if "--out" in argv:
        i = argv.index("--out"); out_dir = argv[i + 1]; del argv[i:i + 2]
    if "--jobs" in argv:
        i = argv.index("--jobs"); jobs = int(argv[i + 1]); del argv[i:i + 2]
    dirs = [d.rstrip("/") for d in argv]
    files = []
    for d in dirs:
        for f in sorted(os.listdir(os.path.join(SRC, d))):
            if f.endswith((".cpp", ".h")):
                files.append((os.path.join(SRC, d, f), dirs))
    with mp.Pool(jobs) as pool:
        results = pool.map(collect, files, chunksize=1)
    types, bodies, errors = {}, {}, []
    for t, b, e in results:
        for k, v in t.items():
            types.setdefault(k, v)
        bodies.update({k: v for k, v in b.items() if k not in bodies})
        errors += e
    # Rust names: nested items become Outer_Inner; already-ported Rust types (known_types.json) win.
    ported = json.load(open(os.path.join(HERE, "known_types.json"))) if os.path.exists(os.path.join(HERE, "known_types.json")) else {}
    known, taken, dupes = {n: n for n in ported}, set(ported), []
    for v in types.values():
        if v["kind"] == "fn":
            continue
        rust = f"{v['parent']}_{v['name']}" if v.get("parent") else v["name"]
        if v["name"] in ported and not v.get("parent"):
            v["skip"] = True  # already ported by hand in utils/foreign
            continue
        if rust in taken:
            dupes.append(f"{rust} ({v['file']})"); v["skip"] = True
            continue
        taken.add(rust)
        v["rust"] = rust
        known[f"{v['parent']}::{v['name']}" if v.get("parent") else v["name"]] = rust
        if v.get("parent") and v["name"] not in known:
            known[v["name"]] = rust  # bare use inside the outer class
    tm = TypeMapper(known)
    by_file = {}
    for v in types.values():
        if not v.get("skip"):
            by_file.setdefault(v["file"].rsplit(".", 1)[0], []).append(v)
    written, mods = [], {}
    for base, items in sorted(by_file.items()):
        parts = [snake(p) for p in base.replace("-", "_").split("/")]
        rust_path = os.path.join(out_dir, *parts[:-1], parts[-1] + ".rs")
        os.makedirs(os.path.dirname(rust_path), exist_ok=True)
        text = f"//! Port of {base}.{{h,cpp}} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.\n"
        text += "#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]\nuse crate::prelude::*;\n\n"
        for v in sorted(items, key=lambda x: ("enum typedef class fn".split().index(x["kind"]), x["name"])):
            if v["kind"] == "enum":
                text += emit_enum(v)
            elif v["kind"] == "typedef":
                text += f"/// C++ typedef `{v['target']} {(v['parent'] + '::') if v['parent'] else ''}{v['name']}`\npub type {v['rust']} = {tm.map(v['target'])};\n\n"
            elif v["kind"] == "class":
                text += emit_class(v, tm, bodies)
            else:
                text += emit_fn(v, tm, bodies)
        open(rust_path, "w", encoding="utf8", newline="\n").write(text)
        written.append(rust_path)
        for i in range(len(parts)):  # register module chain
            mods.setdefault("/".join(parts[:i]), set()).add(parts[i])
    # mod.rs files, lib.rs and prelude
    for parent, children in mods.items():
        if parent == "":
            continue
        open(os.path.join(out_dir, *parent.split("/"), "mod.rs"), "w", encoding="utf8", newline="\n").write(
            "".join(f"pub mod {c};\n" for c in sorted(children)))
    lib = "//! GENERATED by tools/skeleton/gen.py: module tree of the SUMO port.\n#![allow(clippy::all)]\npub mod prelude;\n"
    lib += "".join(f"pub mod {c};\n" for c in sorted(mods.get("", ())))
    open(os.path.join(out_dir, "lib.rs"), "w", encoding="utf8", newline="\n").write(lib)
    prelude = ("//! GENERATED: every ported/skeleton type, so generated modules can name each other.\n"
               "#![allow(unused_imports, ambiguous_glob_reexports)]\n"
               "pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};\npub use std::rc::Rc;\n"
               "/// Placeholder for a C++ type the generator could not map; replace when porting the code that uses it.\n"
               "pub type Unmapped = ();\n")
    prelude += "".join(f"pub use {path}::{name};\n" for name, path in sorted(ported.items()))
    prelude += "pub use utils::common::time::*;\npub use utils::common::sumo_vehicle_class::*;\npub use utils::xml::sumo_xml_definitions::*;\n"
    prelude += "".join(f"pub use crate::{w}::*;\n" for w in sorted(
        os.path.relpath(p, out_dir)[:-3].replace(os.sep, "::") for p in written))
    open(os.path.join(out_dir, "prelude.rs"), "w", encoding="utf8", newline="\n").write(prelude)
    report = {"files": len(written), "types": sum(1 for v in types.values() if v["kind"] != "fn"),
              "functions": sum(1 for v in types.values() if v["kind"] == "fn"),
              "unmapped_types": sorted(tm.unmapped), "duplicate_names": dupes, "errors": errors[:50]}
    json.dump(report, open(os.path.join(HERE, "last_report.json"), "w"), indent=1)
    print(f"wrote {len(written)} files, {report['types']} types, {report['functions']} free functions, "
          f"{len(tm.unmapped)} unmapped C++ types, {len(dupes)} duplicate names, {len(errors)} parse errors")


if __name__ == "__main__":
    main()
