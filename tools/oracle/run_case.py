"""Run reference and candidate SUMO on one case directory and compare outputs.

Case directory: routes.rou plus net.xml, or nod.xml + edg.xml (then netconvert builds net.xml per side).
Prints one line per stage: "<stage>: PASS|FAIL|ERROR <detail>". Exit 0 only if every stage passes.

Usage:
  python run_case.py CASE_DIR [--ref-sumo sumo] [--cand-sumo sumo]
                     [--ref-netconvert netconvert] [--cand-netconvert netconvert]
                     [--out OUT_DIR] [--end SECONDS] [--seed N]
"""

import argparse
import os
import shutil
import subprocess
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import compare_sim  # noqa: E402

TIMEOUT_S = 3600


def stage(name, ok, detail=""):
    status = "PASS" if ok is True else ("FAIL" if ok is False else "ERROR")
    print(f"{name}: {status}{(' ' + detail) if detail else ''}")
    return status


def tail(proc):
    return (proc.stderr or proc.stdout or f"exit {proc.returncode}").strip()[-300:]


def fresh_dir(path):
    shutil.rmtree(path, ignore_errors=True)
    path.mkdir(parents=True)
    return path


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("case", type=Path)
    p.add_argument("--ref-sumo", default="sumo")
    p.add_argument("--cand-sumo", default="sumo")
    p.add_argument("--ref-netconvert", default="netconvert")
    p.add_argument("--cand-netconvert", default="netconvert")
    p.add_argument("--out", type=Path, default=None)
    p.add_argument("--end", type=int, default=3600)
    p.add_argument("--seed", type=int, default=42)
    opts = p.parse_args(argv)

    case = opts.case.resolve()
    out = (opts.out or case / "oracle_out").resolve()
    env = dict(os.environ)
    if not env.get("SUMO_HOME"):
        stage("setup", None, "SUMO_HOME is not set")
        return 1
    if not (case / "routes.rou").exists():
        stage("setup", None, f"{case / 'routes.rou'} not found")
        return 1

    sides = {"ref": (opts.ref_sumo, opts.ref_netconvert), "cand": (opts.cand_sumo, opts.cand_netconvert)}
    build_net = (case / "nod.xml").exists() and (case / "edg.xml").exists()
    results = []
    nets = {}

    for side, (_, nc) in sides.items():
        d = fresh_dir(out / side)
        if build_net:
            args = ["-n", str(case / "nod.xml"), "-e", str(case / "edg.xml"), "-o", str(d / "net.xml")]
            try:
                r = subprocess.run([nc, *args], cwd=d, env=env, capture_output=True, text=True, timeout=TIMEOUT_S)
                ok = r.returncode == 0 and (d / "net.xml").exists()
                results.append(stage(f"netconvert-{side}", ok or None, "" if ok else tail(r)))
            except (OSError, subprocess.TimeoutExpired) as e:
                results.append(stage(f"netconvert-{side}", None, str(e)))
            nets[side] = d / "net.xml"
        else:
            nets[side] = case / "net.xml"

    dirs = {side: out / side for side in sides}
    for side, (sumo, _) in sides.items():
        d = dirs[side]
        args = [
            "-n", str(nets[side]),
            "-r", str(case / "routes.rou"),
            "--tripinfo-output", str(d / "tripinfo.xml"),
            "--fcd-output", str(d / "fcd.xml"),
            "--statistic-output", str(d / "statistic.xml"),
            "--precision", "17",
            "--seed", str(opts.seed),
            "--end", str(opts.end),
            "--no-step-log",
        ]
        try:
            r = subprocess.run([sumo, *args], cwd=d, env=env, capture_output=True, text=True, timeout=TIMEOUT_S)
            results.append(stage(f"run-{side}", r.returncode == 0 or None, "" if r.returncode == 0 else tail(r)))
        except (OSError, subprocess.TimeoutExpired) as e:
            results.append(stage(f"run-{side}", None, str(e)))

    runs_ok = all(s == "PASS" for s in results)
    for name, fn, fname in (
        ("tripinfo", compare_sim.compare_tripinfo, "tripinfo.xml"),
        ("fcd", compare_sim.compare_fcd, "fcd.xml"),
    ):
        if not runs_ok:
            results.append(stage(name, None, "skipped: earlier stage failed"))
            continue
        try:
            diff = fn(dirs["ref"] / fname, dirs["cand"] / fname)
            results.append(stage(name, diff is None, diff or ""))
        except (OSError, ET.ParseError) as e:
            results.append(stage(name, None, str(e)))

    return 0 if all(s == "PASS" for s in results) else 1


if __name__ == "__main__":
    sys.exit(main())
