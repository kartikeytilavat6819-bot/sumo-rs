#!/usr/bin/env python3
"""Generate C++ reference outputs for imported TextTest cases.

For every `tools/oracle/cases/tt_*/` dir with a NEW_CASE marker this runs the
official sumo (one WSL session for all cases) with exactly the flags
`make_sim_refs.sh` uses for a new case, and checks that the three XML outputs
are well formed.  Cases whose official run fails (error-exit tests, unsupported
options, ...) get a PENDING marker with the reason and lose their NEW_CASE
marker, so run_all.sh SKIPs them and make_sim_refs.sh does not retry them.

Usage: python tools/oracle/gen_tt_refs.py [--case NAME] [--keep-errors]
"""
from __future__ import annotations

import argparse
import os
import subprocess
import sys
import xml.etree.ElementTree as ET

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(HERE))
CASES = os.path.join(REPO, "tools", "oracle", "cases")
SUMO = os.environ.get("SUMO_ORACLE_BIN", "$HOME/sumo-pristine/bin/sumo")
TIMEOUT = int(os.environ.get("GEN_REF_TIMEOUT", "60"))
# FCD above this is not worth committing (a few high-flow tests produce >70 MB).
FCD_MAX = int(os.environ.get("GEN_FCD_MAX", str(256 * 1024)))


def wsl_path(host: str) -> str:
    p = host.replace("\\", "/")
    if len(p) >= 2 and p[1] == ":":
        p = "/mnt/" + p[0].lower() + p[2:]
    return p


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--case", action="append", default=[], help="only this case (repeatable)")
    ap.add_argument("--keep-errors", action="store_true",
                    help="leave failing cases untouched (do not mark PENDING)")
    args = ap.parse_args(argv)

    names = sorted(n for n in os.listdir(CASES)
                   if n.startswith("tt_") and os.path.isdir(os.path.join(CASES, n)))
    if args.case:
        names = [n for n in names if n in args.case]
    def refs_ok(n):
        d = os.path.join(CASES, n)
        files = [os.path.join(d, f) for f in
                 ("ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml")]
        if not all(os.path.isfile(f) for f in files):
            return False
        try:
            for f in files:
                ET.parse(f)
            return True
        except ET.ParseError:
            return False

    todo = [n for n in names
            if os.path.isfile(os.path.join(CASES, n, "NEW_CASE"))]
    already = [n for n in todo if refs_ok(n)]
    todo = [n for n in todo if not refs_ok(n)]
    print(f"# {len(todo)} cases need refs, {len(already)} already complete "
          f"(of {len(names)} imported)", flush=True)
    # Apply the fcd size policy to cases completed by an earlier run.
    for name in already:
        fcd = os.path.join(CASES, name, "ref.fcd.xml")
        if os.path.getsize(fcd) > FCD_MAX:
            os.remove(fcd)
            os.remove(os.path.join(CASES, name, "NEW_CASE"))
    if not todo:
        print("# done ok=0 bad=0")
        return 0

    root = wsl_path(REPO)
    script = ["#!/bin/bash", f"cd {root}",
              "export SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo",
              f"SUMO={SUMO}"]
    for name in todo:
        d = f"tools/oracle/cases/{name}"
        script.append(
            f"timeout {TIMEOUT} setsid --wait $SUMO "
            f"-n {root}/{d}/ref_simple.net.xml -r {root}/{d}/routes.rou "
            f"$(cat {d}/sim.args) "
            f"--tripinfo-output {root}/{d}/ref.tripinfo.xml "
            f"--fcd-output {root}/{d}/ref.fcd.xml "
            f"--statistic-output {root}/{d}/ref.stat.xml --no-step-log true "
            f"> /tmp/oc49_ref_{name}.log 2>&1; echo RESULT {name} $?"
        )
    inner = os.path.join(REPO, "tools", "oracle", ".gen_tt_inner.sh")
    with open(inner, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("\n".join(script) + "\n")
    try:
        proc = subprocess.run(["wsl", "-e", "bash", wsl_path(inner)],
                              capture_output=True, text=True, timeout=TIMEOUT * len(todo) + 300)
        out = proc.stdout
        if proc.returncode != 0:
            print("# wsl driver stderr: " + (proc.stderr[:300]).replace("\n", " "), flush=True)
    finally:
        try:
            os.remove(inner)
        except OSError:
            pass

    rcs = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[0] == "RESULT" and parts[1] in todo:
            rcs[parts[1]] = parts[2]

    ok = bad = trimmed = 0
    for name in todo:
        case = os.path.join(CASES, name)
        rc = rcs.get(name, "missing")
        reason = None
        if rc != "0":
            log = wsl_path(os.path.join("/tmp", f"oc49_ref_{name}.log"))
            try:
                text = subprocess.run(["wsl", "-e", "cat", log], capture_output=True,
                                      text=True, timeout=30).stdout
            except subprocess.TimeoutExpired:
                text = ""
            reason = f"official sumo exit {rc}: {' '.join(text.split())[:150]}"
        else:
            for f in ("ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml"):
                try:
                    ET.parse(os.path.join(case, f))
                except (ET.ParseError, OSError) as e:
                    reason = f"bad {f}: {e}"
                    break
        if reason is None:
            fcd = os.path.join(case, "ref.fcd.xml")
            if os.path.getsize(fcd) > FCD_MAX:
                os.remove(fcd)
                os.remove(os.path.join(case, "NEW_CASE"))
                trimmed += 1
            ok += 1
        else:
            bad += 1
            if not args.keep_errors:
                with open(os.path.join(case, "PENDING"), "w", encoding="utf-8") as fh:
                    fh.write(reason + "\n")
                try:
                    os.remove(os.path.join(case, "NEW_CASE"))
                except OSError:
                    pass
                for f in ("ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml"):
                    try:
                        os.remove(os.path.join(case, f))
                    except OSError:
                        pass
            print(f"SKIP {name}: {reason[:140]}", flush=True)
    print(f"# done ok={ok} bad={bad} fcd_trimmed={trimmed}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
