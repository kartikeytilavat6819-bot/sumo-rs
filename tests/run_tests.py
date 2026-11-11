"""Run SUMO's texttest suites (tests/sumo/...) against the Rust sumo binary.

Usage: python tests/run_tests.py [filter-substring ...] [--sumo-src PATH] [-v]
Prints pass/fail per test and a pass-rate total.

shortcut: implements only the texttest features the sumo suite uses for file
outputs (copy_test_path, options chaining, comment stripping, float tolerance);
stdout/stderr ("output"/"errors") comparison is skipped until the port prints SUMO's messages.
"""
import os, re, shlex, shutil, subprocess, sys, tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SUMO_SRC = os.environ.get("SUMO_SRC", os.path.join(os.path.dirname(ROOT), "sumo-src"))
BIN = os.path.join(ROOT, "target", "release", "sumo.exe" if os.name == "nt" else "sumo")
TOL = 0.0101
NUM = re.compile(r"-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?")


def read_config(path):
    """Return (copy_paths, collate {key: filename}) from a texttest config file."""
    copies, collate, section = [], {}, None
    for line in open(path, encoding="utf8"):
        line = line.strip()
        if line.startswith("["):
            section = line.strip("[]")
        elif line.startswith("copy_test_path:"):
            copies.append(line.split(":", 1)[1])
        elif section == "collate_file" and ":" in line:
            k, v = line.split(":", 1)
            collate.setdefault(k, v)
    return copies, collate


def strip_comments(text):
    return re.sub(r"<!--.*?-->", "", text, flags=re.S)


def same(expected, actual):
    a, b = [l for l in strip_comments(expected).splitlines() if l.strip()], \
           [l for l in strip_comments(actual).splitlines() if l.strip()]
    if len(a) != len(b):
        return False, f"line count {len(a)} != {len(b)}"
    for i, (x, y) in enumerate(zip(a, b)):
        if x == y:
            continue
        if NUM.sub("#", x) != NUM.sub("#", y):
            return False, f"line {i}: {x.strip()!r} vs {y.strip()!r}"
        for n, m in zip(NUM.findall(x), NUM.findall(y)):
            if abs(float(n) - float(m)) > TOL:
                return False, f"line {i}: {x.strip()!r} vs {y.strip()!r}"
    return True, ""


def find_up(name, start, stop):
    d = start
    while True:
        p = os.path.join(d, name)
        if os.path.exists(p):
            return p
        if os.path.samefile(d, stop):
            return None
        d = os.path.dirname(d)


def run_test(test_dir, suite_root, copies, collate, verbose):
    opts = []
    d, chain = test_dir, []
    while True:  # options files chain root -> leaf
        p = os.path.join(d, "options.sumo")
        if os.path.exists(p):
            chain.append(p)
        if os.path.samefile(d, suite_root):
            break
        d = os.path.dirname(d)
    for p in reversed(chain):
        opts += shlex.split(open(p, encoding="utf8").read())
    with tempfile.TemporaryDirectory() as work:
        for f in os.listdir(test_dir):
            src = os.path.join(test_dir, f)
            if os.path.isfile(src) and not f.endswith(".sumo"):
                shutil.copy(src, work)
        for name in copies:
            src = find_up(name, test_dir, suite_root)
            if src and not os.path.exists(os.path.join(work, name)):
                (shutil.copytree if os.path.isdir(src) else shutil.copy)(src, os.path.join(work, name))
        try:
            r = subprocess.run([BIN] + opts, cwd=work, capture_output=True, text=True, timeout=120)
        except subprocess.TimeoutExpired:
            return False, "timeout"
        checked = 0
        for key, fname in collate.items():
            exp = os.path.join(test_dir, key + ".sumo")
            if key in ("output", "errors") or not os.path.exists(exp):
                continue
            checked += 1
            out = os.path.join(work, fname)
            if not os.path.exists(out):
                return False, f"missing {fname} (exit {r.returncode}) {r.stderr.strip()[:200]}"
            ok, why = same(open(exp, encoding="utf8").read(), open(out, encoding="utf8", errors="replace").read())
            if not ok:
                return False, f"{key}: {why}"
        if checked == 0:
            return None, "no file outputs to compare"
        return True, ""


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("-")]
    verbose = "-v" in sys.argv
    suite_root = os.path.join(SUMO_SRC, "tests", "sumo")
    copies, collate = read_config(os.path.join(suite_root, "config.sumo"))
    passed = failed = 0
    for d, dirs, files in os.walk(suite_root):
        dirs.sort()
        if "options.sumo" not in files or d == suite_root:
            continue
        if any(x.startswith("options.sumo") for x in dirs) or "testsuite.sumo" in files:
            continue  # suite directory, not a test
        rel = os.path.relpath(d, suite_root).replace("\\", "/")
        if args and not any(a in rel for a in args):
            continue
        ok, why = run_test(d, suite_root, copies, collate, verbose)
        if ok is None:
            continue
        passed += ok
        failed += not ok
        if verbose or ok:
            print(("PASS " if ok else "FAIL ") + rel + ("" if ok else "  " + why))
    total = passed + failed
    print(f"\n{passed}/{total} passed ({100 * passed / max(total, 1):.1f}%)")


if __name__ == "__main__":
    main()
