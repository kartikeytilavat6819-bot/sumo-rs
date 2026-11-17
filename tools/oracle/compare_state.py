#!/usr/bin/env python3
"""Compare two <snapshot> state files (--save-state output) of the same simulation.

Usage: python compare_state.py <ours.xml> <reference.xml>

The state bodies must be identical. The <?xml ...?> header, the <snapshot> tag, comments and the
<configuration> block (paths, output names, timestamps) are ignored. Exit status 0 when identical.
"""

import difflib
import sys


def state_body(path):
    body, in_config = [], False
    for line in open(path, encoding="utf-8").read().splitlines():
        s = line.strip()
        if s.startswith(("<configuration", "<sumoConfiguration")):
            in_config = True
        if in_config:
            in_config = not s.startswith(("</configuration", "</sumoConfiguration"))
            continue
        if s.startswith(("<?xml", "<!--", "<snapshot")):
            continue
        body.append(line)
    return body


def main(argv):
    ours, ref = state_body(argv[1]), state_body(argv[2])
    diff = list(difflib.unified_diff(ref, ours, "reference", "ours", lineterm="", n=0))
    changed = sum(1 for l in diff if l[:1] in "+-" and l[:3] not in ("+++", "---"))
    print(f"state lines: ours={len(ours)} reference={len(ref)} differing_lines={changed}")
    for line in diff[:20]:
        print(line)
    return 0 if not diff else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
