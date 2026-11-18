#!/bin/bash
# Gate diagnostic for oc62: spot-check imported TextTest rows from the
# meso / action_step_length / opposite_direction_driving subtrees.
# The gate already runs the whole scoreboard; this only prints focused diffs.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc62-spot
rm -rf "$OUT"; mkdir -p "$OUT"

CASES="
tt_meso_bugs_no_teleporting_backwards
tt_meso_bugs_ticket747
tt_meso_bugs_ticket1502
tt_meso_bugs_ticket1502b
tt_meso_edge_type_default
tt_meso_junction_control_plain
tt_meso_junction_control_no_control
tt_meso_junction_control_tls_coordination_good_control
tt_meso_lane_queue_no_queue
tt_meso_tau_vehicle_tau
tt_meso_teleport_single_jump
tt_action_step_length_CF_dynamics_tau_tau1.0_actionsteplength1.0
tt_action_step_length_LC_dynamics_continuous_lanechange_basic
tt_action_step_length_Sublane_1edge_2lanes
tt_opposite_direction_driving_overtaking_stopped
tt_opposite_direction_driving_slalom_2stopped
tt_opposite_direction_driving_stop_opposite_2edges
"
for c in $CASES; do
  d=tools/oracle/cases/$c
  [ -f "$d/ref.tripinfo.xml" ] || { echo "$c NO_REF"; continue; }
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-72s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-100
    continue
  fi
  d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
  d2=""
  [ -f "$d/ref.fcd.xml" ] && d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
  [ "${d1}${d2}" = "" ] && echo "MATCH" || echo "ti:${d1:-ok} fcd:${d2:-na}"
done
