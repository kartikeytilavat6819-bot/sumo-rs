#!/bin/bash
# Gate diagnostic for oc58: spot-check a sample of imported sublane_model rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-spot
rm -rf "$OUT"; mkdir -p "$OUT"
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_sublane*' | wc -l)
echo "sublane_cases=$total"
for c in \
  tt_sublane_model_bugs_ticket12581 \
  tt_sublane_model_bugs_ticket17483 \
  tt_sublane_model_lateral_speed_lcSigma \
  tt_sublane_model_roundabout_high_traffic \
  tt_sublane_model_bidiEdge_TWLT_TWLT_overtaking \
  tt_sublane_model_motorway_push_for_merge \
  tt_sublane_model_junction_wide_lane_merge2; do
  d=tools/oracle/cases/$c
  [ -d "$d" ] || { echo "$c MISSING"; continue; }
  flags=$(cat "$d/sim.args")
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-62s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-100
  else
    d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
    d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
    echo "ti:${d1:-MATCH} fcd:${d2:-MATCH}"
  fi
done
