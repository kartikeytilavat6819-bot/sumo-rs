#!/bin/bash
# oc123: run the two remaining target complex tests both ways, print first differing lines.
OUT=/tmp/oc123; mkdir -p $OUT
for t in simulation/simulation misc/concurrent; do
  COMPLEX_OUT=$OUT COMPLEX_FILTER="$t" timeout 300 bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL)' | cut -c1-300
  [ -f "$OUT/$t/diff.txt" ] && head -40 "$OUT/$t/diff.txt" | cut -c1-240
done
