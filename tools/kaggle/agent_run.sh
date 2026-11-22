#!/bin/bash
# oc105 measurement: compare our sumo with the official binary on two target rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc105
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_devices_ssm_bugs_ticket13918 tt_devices_rerouting_reroute_congestions_exit_times; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"
    echo "$side exit=$?"
    echo "$side stderr:"; head -20 "$OUT/$c.$side.err"
    echo "$side stdout:"; head -12 "$OUT/$c.$side.out"
  done
done
# oc108: closing_until mode-8 probe (current branch)
C=tools/oracle/cases/tt_extended_rerouter_errors_closing_until_mode8_trip
FLAGS=$(cat "$C/sim.args")
echo "==== oc108 closing_until our"
"$OUR" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" --no-step-log true $FLAGS \
  --tripinfo-output /tmp/oc108_cu_our.tripinfo.xml >/tmp/oc108_cu_our.out 2>/tmp/oc108_cu_our.err
echo "exit=$?"
grep -E 'OC108_RR|OC108_REROUTE|incompatible' /tmp/oc108_cu_our.err | head -40
grep -o 'id="reroute_period"[^>]*arrival="[^"]*"' /tmp/oc108_cu_our.tripinfo.xml | head -2
# oc108: optimizeStops stop-alternative probe (current branch)
C=tools/oracle/cases/tt_extended_rerouter_optimizeStops_grid5x10
FLAGS=$(cat "$C/sim.args")
echo "==== oc108 optimizeStops our"
"$OUR" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" --no-step-log true $FLAGS \
  --tripinfo-output /tmp/oc108_opt_our.tripinfo.xml >/tmp/oc108_opt_our.out 2>/tmp/oc108_opt_our.err
echo "exit=$?"
grep -E 'OC108_REPL|replac|skips unreachable' /tmp/oc108_opt_our.err | head -60
grep -o 'id="t_0"[^>]*arrival="[^"]*"' /tmp/oc108_opt_our.tripinfo.xml | head -2
