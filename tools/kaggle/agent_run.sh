#!/usr/bin/env bash
# oc98 probe (temporary): vehicle/taxi/control_idle and vehicle/distance_after_reroute, official vs ours
OUT=/tmp/oc98; mkdir -p "$OUT"
for t in vehicle/taxi/control_idle vehicle/distance_after_reroute; do
  COMPLEX_OUT="$OUT/cx" COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -a "^PASS\|^FAIL" | head -n 2 | cut -c1-200
  for side in official ours; do
    grep -a -v "^ *Retrying\|UserWarning" "$OUT/cx/$t/$side/console.txt" > "$OUT/c.$side.txt"
  done
  echo "-- $t first 12 diff lines (official < vs ours >):"
  diff "$OUT/c.official.txt" "$OUT/c.ours.txt" | head -n 12 | cut -c1-240 | sed 's/^/     /'
  for side in official ours; do echo "-- $t $side first 14 lines:"; head -n 14 "$OUT/c.$side.txt" | cut -c1-240 | sed 's/^/     /'; done
done
echo "AGENT RUN DONE"
