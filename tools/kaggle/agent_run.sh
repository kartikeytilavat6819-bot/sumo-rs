#!/bin/bash
# oc105 measurement (kept) + oc103 taxi idling diff diagnostic.
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

echo "==== oc103 taxi idling diff"
T=tools/oracle/cases/tt_devices_taxi_idling_randomCircling_rerouting
TEXTRA=$(cat "$T/sim.args")
timeout 300 "$OUR" -n "$T/ref_simple.net.xml" -r "$T/routes.rou" \
  --tripinfo-output "$OUT/taxi.ti.xml" --fcd-output "$OUT/taxi.fcd.xml" \
  --statistic-output "$OUT/taxi.stat.xml" --no-step-log true $TEXTRA >"$OUT/taxi.log" 2>&1
echo "taxi exit=$?"; head -20 "$OUT/taxi.log"
echo "-- tripinfo diff --"
python tools/oracle/compare_sim.py --tripinfo "$T/ref.tripinfo.xml" "$OUT/taxi.ti.xml" 2>&1 | head -20
echo "-- fcd diff --"
python tools/oracle/compare_sim.py --fcd "$T/ref.fcd.xml" "$OUT/taxi.fcd.xml" 2>&1 | head -20
echo "-- stat diff --"
python tools/oracle/compare_stat.py "$T/ref.stat.xml" "$OUT/taxi.stat.xml" 2>&1 | head -20
echo "=== AGENT RUN DONE ==="
