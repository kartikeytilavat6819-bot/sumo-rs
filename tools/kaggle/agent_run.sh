#!/bin/bash
# oc65 temporary measurement: full complex/TraCI oracle + detailed diffs (removed before final commit).
rm -rf /tmp/complex1
COMPLEX_LIMIT=0 COMPLEX_TIMEOUT=12 COMPLEX_OUT=/tmp/complex1 timeout 540 bash tools/oracle/run_complex_suite.sh 2>&1
echo "=== FAIL DIFF DETAILS (first 10)"
n=0
for f in $(find /tmp/complex1 -name diff.txt | sort); do
  n=$((n+1)); [ $n -gt 10 ] && break
  echo "--- $f"; head -22 "$f"
done
echo "=== DONE"
