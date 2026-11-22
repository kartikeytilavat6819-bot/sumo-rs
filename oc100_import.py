import pathlib, subprocess, sys, os, shutil
real = pathlib.Path(r"D:\sumo-src-1.28.0\tests\sumo")
tmp = pathlib.Path(r"C:\Users\karti\AppData\Local\Temp\opencode\oc100_tests\sumo")
if tmp.exists():
    shutil.rmtree(tmp)
lines = [l.strip() for l in pathlib.Path(".oc_import.txt").read_text().splitlines()
         if l.strip() and not l.endswith("#done")]
start = int(sys.argv[1]); end = int(sys.argv[2])
chunk = lines[start:end]
# build minimal ancestor tree with options.sumo files
for rel in chunk:
    parts = rel.replace("tests/sumo/", "").split("/")
    d = real.joinpath(*parts)
    for i in range(1, len(parts) + 1):
        anc = tmp.joinpath(*parts[:i])
        anc.mkdir(parents=True, exist_ok=True)
        srcdir = real.joinpath(*parts[:i-1]) if i > 1 else real
        if srcdir.exists():
            for f in srcdir.iterdir():
                if f.is_file() and not f.name.startswith("testsuite."):
                    shutil.copy2(f, anc / f.name)
    td = tmp.joinpath(*parts)
    td.mkdir(parents=True, exist_ok=True)
    for f in d.iterdir():
        if f.is_file():
            shutil.copy2(f, td / f.name)
    ec = td / "exitcode.sumo"
    if ec.exists():
        ec.unlink()
    ef = td / "errors.sumo"
    if ef.exists():
        txt = ef.read_text(errors="replace")
        kept = [l for l in txt.splitlines(True) if "Quitting (on error)" not in l]
        ef.write_text("".join(kept), newline="")
paths = [str(tmp.joinpath(*l.replace("tests/sumo/", "").split("/"))) for l in chunk]
r = subprocess.run([sys.executable, "tools/oracle/import_texttest.py", "--tests-sumo", str(tmp), *paths],
                   capture_output=True, text=True)
print(r.stdout)
if r.stderr:
    print("STDERR:", r.stderr[:800])
# unpark error cases using the REAL source tree
env = dict(os.environ, SUMO_TESTS=str(real))
names = set()
for l in subprocess.run([sys.executable, "tools/oracle/import_texttest.py", "--tests-sumo", str(tmp), "--dry-run", *paths], capture_output=True, text=True).stdout.splitlines():
    if l.startswith("WOULD IMPORT "):
        names.add(l.split()[2])
for name in sorted(names):
    cdir = pathlib.Path("tools/oracle/cases") / name
    if not (cdir / "CASE.md").exists():
        continue
    src = ""
    for line in (cdir / "CASE.md").read_text(errors="replace").splitlines():
        if line.startswith("Source: `"):
            src = line.split("`")[1]
    realdir = real / src.replace("/", "\\")
    ef = realdir / "errors.sumo"
    quit_err = ef.exists() and "Quitting (on error)" in ef.read_text(errors="replace")
    ec = realdir / "exitcode.sumo"
    nonzero = ec.exists() and ec.read_text(errors="replace").strip() not in ("", "0")
    if quit_err or nonzero:
        u = subprocess.run([sys.executable, "tools/oracle/park_tt_errors.py", "--unpark", name],
                           capture_output=True, text=True, env=env)
        print(u.stdout.strip())
