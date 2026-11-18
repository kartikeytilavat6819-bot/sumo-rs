#!/usr/bin/env python3
"""Generate official C++ refs for NEW_CASE oracle cases using the installed
eclipse-sumo 1.28.0 binary. Mirrors make_sim_refs.sh's main-variant ref run.
Usage: python tools/oracle/genrefs.py --prefix=tt_meso_  |  <case> [<case> ...]"""
import os, subprocess, sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SUMO = r"C:\Users\karti\AppData\Local\Programs\Python\Python313\Lib\site-packages\sumo\bin\sumo.exe"
CASES = os.path.join(REPO, "tools", "oracle", "cases")

def gen(case):
    d = os.path.join(CASES, case)
    if not os.path.isdir(d):
        print("NO DIR", case); return 1
    with open(os.path.join(d, "sim.args"), encoding="utf-8") as f:
        flags = f.read().strip()
    cmd = [SUMO, "-n", os.path.join(d, "ref_simple.net.xml"),
           "-r", os.path.join(d, "routes.rou")]
    cmd += flags.split()
    cmd += ["--tripinfo-output", os.path.join(d, "ref.tripinfo.xml"),
            "--fcd-output", os.path.join(d, "ref.fcd.xml"),
            "--statistic-output", os.path.join(d, "ref.stat.xml"),
            "--no-step-log", "true"]
    r = subprocess.run(cmd, cwd=REPO, stdout=subprocess.PIPE,
                       stderr=subprocess.STDOUT, text=True, timeout=300)
    ok = r.returncode == 0 and all(
        os.path.getsize(os.path.join(d, "ref." + s + ".xml")) > 0
        for s in ("tripinfo", "fcd", "stat"))
    print("%-70s rc=%d %s" % (case, r.returncode, "OK" if ok else "FAIL"))
    if not ok:
        print("\n".join("    " + l for l in r.stdout.splitlines()[-8:]))
    return 0 if ok else 1

if __name__ == "__main__":
    args = sys.argv[1:]
    if len(args) == 1 and args[0].startswith("--prefix="):
        prefix = args[0].split("=", 1)[1]
        cases = sorted(n for n in os.listdir(CASES)
                       if n.startswith(prefix) and os.path.isdir(os.path.join(CASES, n)))
    else:
        cases = args
    bad = 0
    for c in cases:
        bad += gen(c)
    print("TOTAL", len(cases), "cases,", bad, "failed")
    sys.exit(bad)
