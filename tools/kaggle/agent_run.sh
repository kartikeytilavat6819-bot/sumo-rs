#!/bin/bash
# TEMP probe (remove before final commit): distance_after_reroute, official vs ours, first differing lines.
set -u
OUT=/tmp/oc117
rm -rf "$OUT/dist"; mkdir -p "$OUT/dist"
export COMPLEX_OUT="$OUT/complex"
export COMPLEX_TIMEOUT=${COMPLEX_TIMEOUT:-120}
rm -f "$COMPLEX_OUT/vehicle/distance_after_reroute/diff.txt"
COMPLEX_FILTER="vehicle/distance_after_reroute" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL) ' | head -1
d="$COMPLEX_OUT/vehicle/distance_after_reroute/diff.txt"
if [ -f "$d" ]; then
  echo "--- first differing lines for vehicle/distance_after_reroute"
  head -30 "$d"
else
  echo "no diff file: distance_after_reroute matched"
fi
