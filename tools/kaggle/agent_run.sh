#!/bin/bash
# Gate diagnostic for oc58: spot-check the opposite-direction PANIC rows after the fix.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-spot2
rm -rf "$OUT"; mkdir -p "$OUT"
for c in \
  tt_sublane_model_opposite_direction_driving_bugs_ticket8889 \
  tt_sublane_model_opposite_direction_driving_bugs_ticket8890 \
  tt_sublane_model_opposite_direction_driving_stop_opposite_2edges \
  tt_sublane_model_opposite_direction_driving_stop_opposite_2lanes_3opposite \
  tt_sublane_model_opposite_direction_driving_stop_opposite_2lanes_3opposite_late_arrivalLane \
  tt_sublane_model_opposite_direction_driving_stop_opposite_2lanes_opposite; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-72s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-80
  else
    d1=$(python3 tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
    echo "ti:${d1:-MATCH}"
  fi
done
