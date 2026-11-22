from pathlib import Path
base = Path(r"D:\sumo-src-1.28.0\tests\sumo")
lines = [l.strip() for l in Path(".oc_import.txt").read_text().splitlines() if l.strip() and not l.endswith("#done")]
for rel in lines:
    d = base / rel.replace("tests/sumo/", "").replace("/", "\\")
    ec = d / "exitcode.sumo"
    err = d / "errors.sumo"
    hasquit = ("Quitting (on error)" in err.read_text(errors="replace")) if err.exists() else False
    ecv = ec.read_text(errors="replace").strip() if ec.exists() else ""
    print(f"{rel:70s} exitcode={ecv!r} quit={hasquit}")
