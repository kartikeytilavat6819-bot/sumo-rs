#!/usr/bin/env python3
"""Linux/VM twin of gen_tt_refs.py: generate the C++ references of imported tt_* cases with the OFFICIAL sumo,
running it directly ($SUMO_PRISTINE_BIN/sumo) and in parallel. Same flags, same checks and the same PENDING /
fcd-size policy as gen_tt_refs.py (which drives WSL on the laptop).

Usage (repo root, on the gate VM): GEN_REF_JOBS=14 python3 tools/oracle/gen_tt_refs_vm.py [--case NAME ...]
"""
import argparse
import os
import subprocess
import sys
import xml.etree.ElementTree as ET
from concurrent.futures import ThreadPoolExecutor

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
CASES = os.path.join(REPO, "tools", "oracle", "cases")
SUMO = os.path.join(os.environ["SUMO_PRISTINE_BIN"], "sumo")
TIMEOUT = int(os.environ.get("GEN_REF_TIMEOUT", "60"))
FCD_MAX = int(os.environ.get("GEN_FCD_MAX", str(256 * 1024)))
REFS = ("ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml")


def refs_ok(case):
    try:
        for f in REFS:
            ET.parse(os.path.join(case, f))
        return True
    except (ET.ParseError, OSError):
        return False


def run_one(name):
    d = f"tools/oracle/cases/{name}"
    with open(os.path.join(REPO, d, "sim.args"), encoding="utf-8") as fh:
        extra = fh.read().split()
    cmd = [SUMO, "-n", f"{d}/ref_simple.net.xml", "-r", f"{d}/routes.rou", *extra,
           "--tripinfo-output", f"{d}/ref.tripinfo.xml", "--fcd-output", f"{d}/ref.fcd.xml",
           "--statistic-output", f"{d}/ref.stat.xml", "--no-step-log", "true"]
    log = f"/tmp/genref_{name}.log"
    with open(log, "w") as lf:
        try:
            rc = subprocess.run(cmd, cwd=REPO, stdout=lf, stderr=subprocess.STDOUT, timeout=TIMEOUT).returncode
        except subprocess.TimeoutExpired:
            rc = 124
    return name, rc, log


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--case", action="append", default=[])
    args = ap.parse_args()
    names = sorted(n for n in os.listdir(CASES) if n.startswith("tt_") and os.path.isdir(os.path.join(CASES, n)))
    if args.case:
        names = [n for n in names if n in args.case]
    todo = [n for n in names if os.path.isfile(os.path.join(CASES, n, "NEW_CASE")) and not refs_ok(os.path.join(CASES, n))]
    print(f"# {len(todo)} cases need refs (of {len(names)} imported)", flush=True)
    ok = bad = trimmed = 0
    with ThreadPoolExecutor(int(os.environ.get("GEN_REF_JOBS", "8"))) as ex:
        for name, rc, log in ex.map(run_one, todo):
            case = os.path.join(CASES, name)
            reason = None
            if rc != 0:
                with open(log, encoding="utf-8", errors="replace") as fh:
                    reason = f"official sumo exit {rc}: {' '.join(fh.read().split())[:150]}"
            elif not refs_ok(case):
                reason = "official sumo wrote an unreadable reference"
            if reason is None:
                ok += 1
                fcd = os.path.join(case, "ref.fcd.xml")
                if os.path.getsize(fcd) > FCD_MAX:
                    os.remove(fcd)
                    os.remove(os.path.join(case, "NEW_CASE"))
                    trimmed += 1
            else:
                bad += 1
                with open(os.path.join(case, "PENDING"), "w", encoding="utf-8") as fh:
                    fh.write(reason + "\n")
                for f in ("NEW_CASE",) + REFS:
                    try:
                        os.remove(os.path.join(case, f))
                    except OSError:
                        pass
    print(f"# done ok={ok} bad={bad} fcd_trimmed={trimmed}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
