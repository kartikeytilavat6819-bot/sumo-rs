#!/usr/bin/env bash
# oc98 probe (temporary): vehicle/taxi/control_idle official vs ours, first differing lines
OUT=/tmp/oc98; mkdir -p "$OUT"
t=vehicle/taxi/control_idle
COMPLEX_OUT="$OUT/cx" COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -a "^PASS\|^FAIL" | head -n 2 | cut -c1-200
for side in official ours; do
  grep -a -v "^ *Retrying\|UserWarning" "$OUT/cx/$t/$side/console.txt" > "$OUT/ci.$side.txt"
done
echo "-- first 12 diff lines (official vs ours):"
diff "$OUT/ci.official.txt" "$OUT/ci.ours.txt" | head -n 12 | cut -c1-260 | sed 's/^/     /'
echo "-- official lines 28-34:"; sed -n 28,34p "$OUT/ci.official.txt" | cut -c1-260 | sed 's/^/     /'
echo "-- ours lines 28-34:";     sed -n 28,34p "$OUT/ci.ours.txt" | cut -c1-260 | sed 's/^/     /'
echo "-- ours tail:"; tail -n 8 "$OUT/ci.ours.txt" | cut -c1-260 | sed 's/^/     /'
echo "-- official tail:"; tail -n 8 "$OUT/ci.official.txt" | cut -c1-260 | sed 's/^/     /'
echo "AGENT RUN DONE"
