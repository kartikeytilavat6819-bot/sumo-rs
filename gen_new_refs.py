#!/usr/bin/env python3
"""Generate ref.*.xml for the named NEW_CASE dirs using the official sumo on PATH.

Usage: python gen_new_refs.py <case-name>...
"""
import os
import subprocess
import sys

root = os.path.dirname(os.path.abspath(__file__))
cases = os.path.join(root, "tools", "oracle", "cases")

for name in sys.argv[1:]:
    d = os.path.join(cases, name)
    if not os.path.isfile(os.path.join(d, "NEW_CASE")):
        print(name, "no NEW_CASE marker, skipped")
        continue
    args_file = os.path.join(d, "sim.args")
    flags = open(args_file).read().strip() if os.path.isfile(args_file) else ""
    cmd = (["sumo", "-n", os.path.join(d, "ref_simple.net.xml"),
            "-r", os.path.join(d, "routes.rou")]
           + flags.split()
           + ["--tripinfo-output", os.path.join(d, "ref.tripinfo.xml"),
              "--fcd-output", os.path.join(d, "ref.fcd.xml"),
              "--statistic-output", os.path.join(d, "ref.stat.xml"),
              "--no-step-log", "true"])
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    print(name, "exit", r.returncode,
          "" if r.returncode == 0 else (r.stdout + r.stderr)[-300:].replace("\n", " | "))
    if r.returncode != 0:
        sys.exit(1)
    os.remove(os.path.join(d, "NEW_CASE"))
