#!/usr/bin/env python3
"""Import the input files of the official TraCI test suite into this repo.

TextTest reference outputs (*.traci) are deliberately NOT imported: the oracle
compares our sumo with the official sumo at run time instead.  Only files that
a test needs to *run* are copied, preserving the directory hierarchy so that
inputs resolving up the parent chain keep working.
"""
import os
import shutil
import sys

SRC = os.environ.get(
    "SUMO_SRC_TESTS", "C:/Users/karti/sumo-src/tests/traci")
DST = os.path.join(os.path.dirname(os.path.abspath(__file__)), "traci_inputs")

KEEP_NAMES = {"options.traci", "testclient.prog"}
KEEP_EXT = {".xml", ".sumo", ".netgcfg", ".netccfg", ".txt", ".cfg"}
SKIP_NAMES = {"config.traci"}


def wanted(name):
    if name in SKIP_NAMES:
        return False
    if name in KEEP_NAMES:
        return True
    return os.path.splitext(name)[1].lower() in KEEP_EXT


def main():
    if not os.path.isdir(SRC):
        print("import_traci_tests: source not found: " + SRC, file=sys.stderr)
        return 1
    n = 0
    for root, dirs, files in os.walk(SRC):
        for name in files:
            if not wanted(name):
                continue
            src = os.path.join(root, name)
            rel = os.path.relpath(src, SRC)
            dst = os.path.join(DST, rel)
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.copyfile(src, dst)
            n += 1
    print("import_traci_tests: copied %d files to %s" % (n, DST))
    return 0


if __name__ == "__main__":
    sys.exit(main())
