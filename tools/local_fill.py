"""Fill short todo!() bodies in the sumo_core skeleton with a LOCAL model served by vLLM.

Each todo!("port Class::method") in the generated skeleton has the C++ body as `//` comments above
the Rust signature. For methods whose C++ body is short, ask the local model for the Rust body,
insert it, then `cargo check` the crate; any method that fails to compile is reverted to todo!(),
so only compiling code is kept. Runs in its own git worktree/branch (no effect on main until merged).

Usage (Windows python, vLLM server running in WSL on :8000):
    python tools/local_fill.py --worktree ../sumo-rs-local --max-lines 15 [--exclude a.rs,b.rs] [--limit-files N]
"""
import argparse, json, os, re, subprocess, sys, urllib.request

API = os.environ.get("LOCAL_LLM_URL", "http://localhost:8000/v1/chat/completions")
MODEL = os.environ.get("LOCAL_LLM_MODEL", "")  # empty -> first model the server reports
CARGO = os.path.join(os.path.expanduser("~"), ".cargo", "bin", "cargo.exe")  # Windows: subprocess ignores env PATH for lookup
CARGO_ENV = dict(os.environ, CARGO_TARGET_DIR=r"C:\Users\karti\sumo-rs-target-local", CARGO_BUILD_JOBS="8")
CARGO_ENV["PATH"] = os.path.expanduser(r"~\.cargo\bin") + os.pathsep + CARGO_ENV.get("PATH", "")

SYSTEM = """You translate one C++ method body from Eclipse SUMO into Rust, faithfully.
Rules:
- Output ONLY the Rust statements that go inside the function body. No signature, no braces around it, no prose, no markdown fences.
- C++ pointers are `Ptr<T>`: call methods with `p.method()`, mutate with `p.m().method()`, null check `p.is_null()`, null value `Ptr::null()`.
- Member fields keep their C++ names: `myFoo` -> `self.myFoo`. Methods are snake_case: `getSpeed()` -> `get_speed()`.
- Keep the exact order of floating-point operations. Don't simplify. Keep every branch.
- `std::string` -> `String`/`&str`; `std::vector` -> `Vec`; `NULL`/`nullptr` -> `Ptr::null()`.
- Return values: last expression or `return x;` as in C++.
"""


def server_model():
    with urllib.request.urlopen(API.replace("/chat/completions", "/models"), timeout=10) as r:
        return json.load(r)["data"][0]["id"]


def ask(model, cpp, sig, fields):
    user = f"C++ method:\n{cpp}\n\nRust signature:\n{sig}\n\nStruct fields available on self:\n{fields}\n\nRust body:"
    req = {"model": model, "temperature": 0, "max_tokens": 700,
           "messages": [{"role": "system", "content": SYSTEM}, {"role": "user", "content": user}]}
    data = json.dumps(req).encode()
    with urllib.request.urlopen(urllib.request.Request(API, data, {"Content-Type": "application/json"}), timeout=180) as r:
        out = json.load(r)["choices"][0]["message"]["content"]
    out = re.sub(r"^```\w*\n|```\s*$", "", out.strip(), flags=re.M).strip()
    return out


def methods(lines, max_lines):
    """Yield (todo_line_index, cpp_text, signature_line, impl_struct) for short todo!() methods."""
    impl = None
    for i, l in enumerate(lines):
        m = re.match(r"impl(?:<[^>]*>)?\s+(\w+)", l)
        if m:
            impl = m.group(1)
        if 'todo!("port ' not in l:
            continue
        j = i - 1
        while j >= 0 and not lines[j].lstrip().startswith("pub fn"):
            j -= 1
        sig = lines[j].strip()
        k = j - 1
        while k >= 0 and lines[k].lstrip().startswith("#["):
            k -= 1
        body = []
        while k >= 0 and lines[k].lstrip().startswith("//") and not lines[k].lstrip().startswith("///"):
            body.append(lines[k].lstrip()[2:].rstrip()[1:] if lines[k].lstrip()[2:3] == " " else lines[k].lstrip()[2:])
            k -= 1
        body.reverse()
        code = [b for b in body if b.strip() not in ("", "{", "}")]
        if body and len(code) <= max_lines:
            yield i, "\n".join(body), sig, impl


def struct_fields(text, name):
    m = re.search(r"pub struct " + re.escape(name) + r"\b[^{]*\{(.*?)\n\}", text, re.S)
    if not m:
        return "(unknown)"
    return "\n".join(l.strip() for l in m.group(1).splitlines() if l.strip().startswith("pub "))[:3000]


def cargo_errors(wt):
    r = subprocess.run([CARGO, "check", "-q", "-p", "sumo_core", "--message-format", "short"],
                       cwd=wt, env=CARGO_ENV, capture_output=True, text=True)
    errs = []
    for l in (r.stdout + r.stderr).splitlines():
        m = re.match(r"(crates[\\/][^:]+):(\d+):\d+: error", l)
        if m:
            errs.append((m.group(1).replace("\\", "/"), int(m.group(2))))
    return errs


def fill_file(wt, rel, model, max_lines):
    path = os.path.join(wt, rel)
    text = open(path, encoding="utf8").read()
    lines = text.split("\n")
    original = {}  # line index of filled body start -> (original todo line, number of inserted lines)
    edits = []
    for i, cpp, sig, impl in methods(lines, max_lines):
        try:
            body = ask(model, cpp, sig, struct_fields(text, impl or ""))
        except Exception as e:  # noqa: BLE001
            print(f"  model error on {sig[:60]}: {e}")
            continue
        if not body or "todo!(" in body:
            continue
        indent = re.match(r"\s*", lines[i]).group(0)
        edits.append((i, [indent + b for b in body.splitlines()]))
    if not edits:
        return 0
    # apply bottom-up so indices stay valid; remember spans for reverting
    spans = []
    for i, new in sorted(edits, reverse=True):
        spans.append((i, lines[i], len(new)))
        lines[i:i + 1] = new
    spans.sort()
    # compute final positions of each span (top-down shift)
    shift, final = 0, []
    for i, orig, n in spans:
        final.append((i + shift, orig, n))
        shift += n - 1
    open(path, "w", encoding="utf8", newline="\n").write("\n".join(lines))
    for _ in range(4):  # revert failing methods until the file compiles
        bad = {ln - 1 for f, ln in cargo_errors(wt) if f == rel.replace("\\", "/")}
        if not bad:
            break
        lines = open(path, encoding="utf8").read().split("\n")
        reverted = {s for s, _, n in final if any(s <= b < s + n for b in bad)}
        for start, orig, n in sorted(final, reverse=True):  # bottom-up keeps upper indices valid
            if start in reverted:
                lines[start:start + n] = [orig]
        shift, new_final = 0, []  # spans below a reverted span move up by (n - 1)
        for start, orig, n in sorted(final):
            if start in reverted:
                shift += n - 1
            else:
                new_final.append((start - shift, orig, n))
        final = new_final
        open(path, "w", encoding="utf8", newline="\n").write("\n".join(lines))
    if any(f == rel.replace("\\", "/") for f, _ in cargo_errors(wt)):
        subprocess.run(["git", "checkout", "--", rel], cwd=wt)  # give up on this file, keep main clean
        return 0
    return len(final)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--worktree", required=True)
    ap.add_argument("--max-lines", type=int, default=15)
    ap.add_argument("--exclude", default="")
    ap.add_argument("--limit-files", type=int, default=0)
    ap.add_argument("--state", default="", help="file listing already-processed files (skipped, appended to)")
    a = ap.parse_args()
    done = set()
    if a.state and os.path.exists(a.state):
        done = {l.strip() for l in open(a.state, encoding="utf8") if l.strip()}
    wt = os.path.abspath(a.worktree)
    model = MODEL or server_model()
    excl = {e.strip() for e in a.exclude.split(",") if e.strip()}
    files = []
    for d, _, fs in os.walk(os.path.join(wt, "crates", "sumo_core", "src")):
        for f in fs:
            if f.endswith(".rs"):
                rel = os.path.relpath(os.path.join(d, f), wt).replace("\\", "/")
                if os.path.basename(rel) not in excl and rel not in done and 'todo!("port ' in open(os.path.join(d, f), encoding="utf8").read():
                    files.append(rel)
    files.sort()
    if a.limit_files:
        files = files[:a.limit_files]
    total = 0
    for rel in files:
        n = fill_file(wt, rel, model, a.max_lines)
        total += n
        if n:
            subprocess.run(["git", "add", rel], cwd=wt)
            subprocess.run(["git", "commit", "-qm", f"Local model fill: {n} methods in {os.path.basename(rel)}"], cwd=wt)
        print(f"{rel}: {n} methods kept (total {total})", flush=True)
        if a.state:
            open(a.state, "a", encoding="utf8").write(rel + "\n")
    print(f"REMAINING_FILES={1 if files else 0}", flush=True)


if __name__ == "__main__":
    main()
