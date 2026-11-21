#!/bin/bash
# oc98 rail-rerouter diagnostics. Run from the repo root after the build.
# Prints, for each of the two target cases and both binaries, the warnings, the
# vehroute replacements (who rerouted the train and to which edges), and the
# tripinfo line. This shows whether the on-init reroute through MSTriggeredRerouter
# fires on our side and which route the train actually takes.
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
REF=${SUMO_PRISTINE_BIN:?SUMO_PRISTINE_BIN not set}/sumo
OUT=${OUT:-/tmp/agent_run_oc98}
mkdir -p "$OUT"

for case in tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait; do
  C=tools/oracle/cases/$case
  echo "################ $case"
  for side in ours pristine; do
    if [ "$side" = ours ]; then BIN=$OURS; else BIN=$REF; fi
    timeout 300 "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
      $(cat "$C/sim.args") --no-step-log true \
      --tripinfo-output "$OUT/$case.$side.ti.xml" \
      --vehroute-output "$OUT/$case.$side.vr.xml" \
      --fcd-output "$OUT/$case.$side.fcd.xml" \
      >"$OUT/$case.$side.log" 2>&1
    echo "== $side exit=$?"
    echo "-- warnings/first log lines:"
    grep -E 'Warning|Error|skips stop|Terminating|no valid route|could not' "$OUT/$case.$side.log" | head -10
    echo "-- vehroutes (route elements):"
    grep -E '<route' "$OUT/$case.$side.vr.xml" | head -10
    echo "-- tripinfo:"
    grep '<tripinfo ' "$OUT/$case.$side.ti.xml" | head -3
    echo "-- first fcd timestamps (vehicle position/edge):"
    grep -E '<timestep ' "$OUT/$case.$side.fcd.xml" | head -4
    grep -E '<vehicle ' "$OUT/$case.$side.fcd.xml" | head -4
  done
done
echo "OC98 DIAGNOSTIC DONE"
