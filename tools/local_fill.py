"""Fill short todo!() bodies in the sumo_core skeleton with a LOCAL model served by vLLM.

Each todo!("port Class::method") in the generated skeleton has the C++ body as `//` comments above
the Rust signature. For methods whose C++ body is short, ask the local model for the Rust body,
insert it, then `cargo check` the crate; any method that fails to compile is reverted to todo!(),
so only compiling code is kept. Runs in its own git worktree/branch (no effect on main until merged).

Usage (Windows python, vLLM server running in WSL on :8000):
    python tools/local_fill.py --worktree ../sumo-rs-local --max-lines 15 [--exclude a.rs,b.rs] [--limit-files N]
"""
import argparse, json, os, re, subprocess, sys, time, urllib.request
from concurrent.futures import ThreadPoolExecutor

API = os.environ.get("LOCAL_LLM_URL", "http://localhost:8000/v1/chat/completions")
MODEL = os.environ.get("LOCAL_LLM_MODEL", "")  # empty -> first model the server reports
# LLM_BACKEND=opencode: OpenCode's free models only work through its CLI (the raw API refuses them)
BACKEND = os.environ.get("LLM_BACKEND", "api")
OC_EXE = os.path.expandvars(r"%APPDATA%\npm\node_modules\@opencode\cli\bin\opencode.exe")
OC_MODELS = os.environ.get("OC_MODELS", "opencode/deepseek-v4.1-flash-free,opencode/glm-5.3-flash-free,"
                           "opencode/minimax-m3.1-flash-free").split(",")
OC_CWD = os.environ.get("OC_CWD", r"C:\Users\karti\oc-sandbox")  # empty dir: the agent has nothing to touch
WORKERS = int(os.environ.get("LLM_WORKERS", "1" if BACKEND == "api" else "6"))
FAILED = []  # files where the model errored (not marked done, so they are retried)
SIGS = {}  # snake_case fn name -> ["Type::pub fn ...", ...] across the workspace (filled in main)
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


def snake(name):  # same rule as tools/skeleton/gen.py
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name)
    s = re.sub(r"(?<=[A-Z])([A-Z][a-z])", r"_\1", s)
    return re.sub(r"\W", "_", s).lower().strip("_")


def index_signatures(wt):
    for d, _, fs in os.walk(os.path.join(wt, "crates")):
        for f in fs:
            if not f.endswith(".rs"):
                continue
            impl = ""
            for l in open(os.path.join(d, f), encoding="utf8", errors="replace"):
                m = re.match(r"impl(?:<[^>]*>)?\s+(?:\w+\s+for\s+)?(\w+)", l)
                if m:
                    impl = m.group(1)
                m = re.match(r"\s*pub fn (\w+)", l)
                if m:
                    SIGS.setdefault(m.group(1), []).append(f"{impl}::{l.strip()[:220]}")


def hints(cpp):
    out = []
    for n in sorted(set(re.findall(r"\b([A-Za-z_]\w*)\s*\(", cpp))):
        out += SIGS.get(snake(n), [])[:4]
    return "\n".join(out[:60]) or "(none found)"


def complete(model, system, user):
    if BACKEND != "opencode":
        req = {"model": model, "temperature": 0, "max_tokens": 700,
               "messages": [{"role": "system", "content": system}, {"role": "user", "content": user}]}
        data = json.dumps(req).encode()
        with urllib.request.urlopen(urllib.request.Request(API, data, {"Content-Type": "application/json"}), timeout=180) as r:
            return json.load(r)["choices"][0]["message"]["content"]
    prompt = system + "\nPut the body in ONE ```rust code block and write nothing else.\n\n" + user
    for attempt in range(len(OC_MODELS) * 2):  # on errors/limits move to the next free model
        m = OC_MODELS[attempt % len(OC_MODELS)]
        try:
            r = subprocess.run([OC_EXE, "run", "--agent", "plan", "--archived", "--format", "json", "-m", m],
                               input=prompt, cwd=OC_CWD, capture_output=True, text=True, encoding="utf8", timeout=300)
            text = "".join(json.loads(l)["part"].get("text", "") for l in r.stdout.splitlines()
                           if l.startswith("{") and '"type":"text"' in l)
        except Exception:  # noqa: BLE001
            text = ""
        if text.strip():
            return text
        time.sleep(20)
    raise RuntimeError("all OpenCode free models failed (usage limit?)")


def ask(model, cpp, sig, fields):
    user = (f"C++ method:\n{cpp}\n\nRust signature:\n{sig}\n\nStruct fields available on self:\n{fields}\n\n"
            f"Existing Rust signatures of the functions it calls (use these exact names):\n{hints(cpp)}\n\nRust body:")
    out = complete(model, SYSTEM, user)
    block = re.search(r"```\w*\n(.*?)```", out, re.S)  # models often add prose after the code
    return (block.group(1) if block else out).strip()


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
    """[(file, line, message)] for every compile error."""
    r = subprocess.run([CARGO, "check", "-q", "-p", "sumo_core", "--message-format", "short"],
                       cwd=wt, env=CARGO_ENV, capture_output=True, text=True)
    errs = []
    for l in (r.stdout + r.stderr).splitlines():
        m = re.match(r"(crates[\\/][^:]+):(\d+):\d+: (error.*)", l)
        if m:
            errs.append((m.group(1).replace("\\", "/"), int(m.group(2)), m.group(3)))
    return errs


def rewrite(lines, spans, new_bodies):
    """Replace span bodies (start -> new line list; None = revert to the todo!() line).
    spans: [dict(start, n, orig, ...)] sorted by start. Returns the updated span list."""
    for sp in sorted(spans, key=lambda s: s["start"], reverse=True):  # bottom-up keeps indices valid
        if sp["start"] in new_bodies:
            new = new_bodies[sp["start"]] or [sp["orig"]]
            lines[sp["start"]:sp["start"] + sp["n"]] = new
            sp["new_n"] = len(new)
    out, shift = [], 0
    for sp in sorted(spans, key=lambda s: s["start"]):
        changed = sp["start"] in new_bodies
        reverted = changed and new_bodies[sp["start"]] is None
        start = sp["start"] + shift
        n = sp.pop("new_n", sp["n"])
        shift += n - sp["n"]
        if not reverted:
            out.append(dict(sp, start=start, n=n))
    return out


def fill_file(wt, rel, model, max_lines):
    path = os.path.join(wt, rel)
    rel = rel.replace("\\", "/")
    text = open(path, encoding="utf8").read()
    lines = text.split("\n")
    spans, new_bodies = [], {}
    jobs = [(i, cpp, sig, struct_fields(text, impl or "")) for i, cpp, sig, impl in methods(lines, max_lines)]

    def one(job):
        try:
            return ask(model, job[1], job[2], job[3])
        except Exception as e:  # noqa: BLE001
            print(f"  model error on {job[2][:60]}: {e}", flush=True)
            FAILED.append(rel)
            return ""
    with ThreadPoolExecutor(WORKERS) as ex:
        answers = list(ex.map(one, jobs))
    for (i, cpp, sig, fields), body in zip(jobs, answers):
        if not body or "todo!(" in body:
            continue
        indent = re.match(r"\s*", lines[i]).group(0)
        spans.append(dict(start=i, n=1, orig=lines[i], cpp=cpp, sig=sig, fields=fields, indent=indent))
        new_bodies[i] = [indent + b for b in body.splitlines()]
    if not spans:
        return 0
    spans = rewrite(lines, spans, new_bodies)
    open(path, "w", encoding="utf8", newline="\n").write("\n".join(lines))
    # one retry per failing method, showing the model its compiler errors
    errs = [(ln - 1, msg) for f, ln, msg in cargo_errors(wt) if f == rel]
    failing = []
    for sp in spans:
        mine = [msg for ln, msg in errs if sp["start"] <= ln < sp["start"] + sp["n"]]
        if mine:
            failing.append((sp, "\n".join(l.strip() for l in lines[sp["start"]:sp["start"] + sp["n"]]), mine))

    def fix(item):
        sp, prev, mine = item
        try:
            return ask(model, sp["cpp"] + "\n\nYour previous Rust body:\n" + prev + "\n\nIt failed to compile:\n"
                       + "\n".join(mine[:6]) + "\nFix it. If a called method doesn't exist, use the closest snake_case name.",
                       sp["sig"], sp["fields"])
        except Exception:  # noqa: BLE001
            return ""
    with ThreadPoolExecutor(WORKERS) as ex:
        fixed = list(ex.map(fix, failing))
    retry = {}
    for (sp, _, _), body in zip(failing, fixed):
        retry[sp["start"]] = [sp["indent"] + b for b in body.splitlines()] if body and "todo!(" not in body else None
    if retry:
        spans = rewrite(lines, spans, retry)
        open(path, "w", encoding="utf8", newline="\n").write("\n".join(lines))
    for _ in range(4):  # revert whatever still fails until the file compiles
        bad = {ln - 1 for f, ln, _ in cargo_errors(wt) if f == rel}
        if not bad:
            break
        revert = {sp["start"]: None for sp in spans if any(sp["start"] <= b < sp["start"] + sp["n"] for b in bad)}
        if not revert:
            break
        spans = rewrite(lines, spans, revert)
        open(path, "w", encoding="utf8", newline="\n").write("\n".join(lines))
    final = spans
    if any(f == rel for f, _, _ in cargo_errors(wt)):
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
    model = MODEL or ("" if BACKEND == "opencode" else server_model())
    index_signatures(wt)
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
        if a.state and rel not in FAILED:
            open(a.state, "a", encoding="utf8").write(rel + "\n")
    print(f"REMAINING_FILES={1 if files else 0}", flush=True)


if __name__ == "__main__":
    main()
