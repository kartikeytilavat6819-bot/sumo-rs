#!/bin/bash
# oc65 temporary measurement: complex/TraCI oracle batch + detailed diffs (removed before final commit).
rm -rf /tmp/complex1
COMPLEX_LIMIT=40 COMPLEX_TIMEOUT=12 COMPLEX_OUT=/tmp/complex1 timeout 540 bash tools/oracle/run_complex_suite.sh 2>&1
echo "=== FAIL DIFF DETAILS (first 8)"
n=0
for f in $(find /tmp/complex1 -name diff.txt | sort); do
  n=$((n+1)); [ $n -gt 8 ] && break
  echo "--- $f"; head -25 "$f"
done
echo "=== DONE"
