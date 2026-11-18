#!/usr/bin/env python3
"""Park imported TextTest rows whose official SUMO run exits non-zero (error-message tests)."""
import os, sys
REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
CASES = os.path.join(REPO, "tools", "oracle", "cases")
reason = ("PARKED: official SUMO 1.28.0 exits non-zero for this TextTest row "
          "(it validates an erroneous additional-file attribute and checks the error message).\n"
          "The run_all harness compares tripinfo/fcd/statistic outputs only, so this row cannot be\n"
          "scored meaningfully. Parked rather than deleted to keep the import provenance.\n")
for case in sys.argv[1:]:
    d = os.path.join(CASES, case)
    with open(os.path.join(d, "CASE.md"), "a", encoding="utf-8", newline="\n") as fh:
        fh.write("\n" + reason)
    with open(os.path.join(d, "PENDING"), "w", encoding="utf-8", newline="\n") as fh:
        fh.write("error-message test: official SUMO exits non-zero; harness compares sim outputs only\n")
    for f in ("NEW_CASE", "ref.tripinfo.xml", "ref.fcd.xml", "ref.stat.xml"):
        try:
            os.remove(os.path.join(d, f))
        except OSError:
            pass
    print("PARKED", case)
