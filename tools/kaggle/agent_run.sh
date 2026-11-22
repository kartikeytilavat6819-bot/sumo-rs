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
    if [ "$side" = ours ]; then DBG=OC98DBG=1; else DBG=; fi
    timeout 300 env $DBG "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
      $(cat "$C/sim.args") --no-step-log true \
      --tripinfo-output "$OUT/$case.$side.ti.xml" \
      --vehroute-output "$OUT/$case.$side.vr.xml" \
      --fcd-output "$OUT/$case.$side.fcd.xml" \
      >"$OUT/$case.$side.log" 2>&1
    echo "== $side exit=$?"
    echo "-- OC98DBG (our side only):"
    grep 'OC98DBG' "$OUT/$case.$side.log" | head -40
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

# ticket17583: 26 vehicles on a circular track with two destProbRerouters.
# Official: none arrive before t=100. Ours: v.25 arrives. Show every tripinfo,
# the flow statistics, and the routes the rerouters assign.
C=tools/oracle/cases/tt_bugs_17xxx_ticket17583
echo "################ tt_bugs_17xxx_ticket17583"
for side in ours pristine; do
  if [ "$side" = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  timeout 300 "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
    $(cat "$C/sim.args") --no-step-log true \
    --tripinfo-output "$OUT/17583.$side.ti.xml" \
    --vehroute-output "$OUT/17583.$side.vr.xml" \
    --statistic-output "$OUT/17583.$side.stat.xml" \
    >"$OUT/17583.$side.log" 2>&1
  echo "== $side exit=$?"
  echo "-- tripinfos:"
  grep '<tripinfo ' "$OUT/17583.$side.ti.xml"
  echo "-- vehicles stat:"
  grep '<vehicles ' "$OUT/17583.$side.stat.xml"
  echo "-- v.25 route replacements (first 12 route tags):"
  grep -E '<route' "$OUT/17583.$side.vr.xml" | head -12
done
echo "OC98 DIAGNOSTIC DONE"
