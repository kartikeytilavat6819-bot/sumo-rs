#!/bin/bash
# Taxi diagnostics: run our sumo and the pristine C++ sumo on the same net/routes.
C=tools/oracle/cases
OURS=${OURS:-/tmp/target/debug/sumo}
PRI=${SUMO_PRISTINE_BIN:-/usr/local/bin}/sumo
OUT=${OUT:-/tmp/agent_run}
mkdir -p "$OUT"
for case in taxi_greedy_shared taxi_route_extension; do
  D="$C/$case"
  ALGO=greedyShared
  [ "$case" = taxi_route_extension ] && ALGO=routeExtension
  for side in ours pristine; do
    if [ "$side" = ours ]; then BIN=$OURS; else BIN=$PRI; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" \
      --tripinfo-output "$OUT/$case.$side.ti.xml" \
      --fcd-output "$OUT/$case.$side.fcd.xml" \
      --statistic-output "$OUT/$case.$side.stat.xml" \
      --no-step-log true --end 600 \
      --device.taxi.dispatch-algorithm "$ALGO" \
      --device.taxi.dispatch-algorithm.output "$OUT/$case.$side.dispatch.xml" \
      > "$OUT/$case.$side.log" 2>&1
    echo "== $case $side exit=$? log: $(grep -m1 -E 'Error|error' "$OUT/$case.$side.log" | cut -c1-120)"
  done
  echo "### $case tripinfo (ours vs ref)"
  python tools/oracle/compare_sim.py --tripinfo "$D/ref.tripinfo.xml" "$OUT/$case.ours.ti.xml" 2>&1 | head -8
  echo "### $case tripinfo (pristine vs ref)"
  python tools/oracle/compare_sim.py --tripinfo "$D/ref.tripinfo.xml" "$OUT/$case.pristine.ti.xml" 2>&1 | head -8
  echo "### $case dispatch ref:"
  grep -E 'dispatch' "$D/ref.dispatch.xml" | sed 's/^ *//'
  echo "### $case dispatch ours:"
  grep -E 'dispatch' "$OUT/$case.ours.dispatch.xml" | sed 's/^ *//'
  echo "### $case dispatch pristine:"
  grep -E 'dispatch' "$OUT/$case.pristine.dispatch.xml" | sed 's/^ *//'
  echo
done
