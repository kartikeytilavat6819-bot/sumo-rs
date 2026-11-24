#!/bin/bash
# oc123: check whether my changes broke vehicle/taxi/control_idle (full gate flagged it)
OUT=/tmp/oc123; mkdir -p $OUT
for t in vehicle/taxi/control_idle; do
  COMPLEX_OUT=$OUT COMPLEX_FILTER="$t" timeout 300 bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL)' | cut -c1-300
  [ -f "$OUT/$t/diff.txt" ] && head -60 "$OUT/$t/diff.txt" | cut -c1-260
done
