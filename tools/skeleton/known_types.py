"""Write known_types.json: {Rust type name: module path} for hand-ported crates (utils, foreign).

gen.py maps C++ types with these names to the existing Rust types instead of generating skeletons.
Run from the repo root after merging new hand ports: python tools/skeleton/known_types.py
"""
import json, os, re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
names = {}
for crate in ("utils", "foreign"):
    src = os.path.join(ROOT, "crates", crate, "src")
    included = {}  # file -> module that include!s it
    for d, _, fs in os.walk(src):
        for f in fs:
            if f.endswith(".rs"):
                for inc in re.findall(r'include!\("([\w./]+)"\)', open(os.path.join(d, f), encoding="utf8").read()):
                    included[os.path.normpath(os.path.join(d, inc))] = os.path.join(d, f)
    for d, _, fs in os.walk(src):
        for f in sorted(fs):
            if not f.endswith(".rs"):
                continue
            p = os.path.join(d, f)
            mod_file = included.get(os.path.normpath(p), p)
            mod = os.path.relpath(mod_file, src)[:-3].replace(os.sep, "::")
            mod = re.sub(r"(::)?\bmod$", "", re.sub(r"^lib$", "", mod))
            path = crate + ("::" + mod if mod else "")
            for m in re.finditer(r"^pub (struct|enum|type|trait) (\w+)", open(p, encoding="utf8").read(), re.M):
                names.setdefault(m.group(2), ("trait:" if m.group(1) == "trait" else "") + path)
json.dump(names, open(os.path.join(ROOT, "tools", "skeleton", "known_types.json"), "w"), indent=0, sort_keys=True)
print(len(names), "known types")
