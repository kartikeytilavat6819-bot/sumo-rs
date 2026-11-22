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

echo "==== oc103 loaded-weights route comparison"
W=tools/oracle/cases/tt_devices_rerouting_weights_init_with_loaded_weights
WFLAGS=$(cat "$W/sim.args")
for c in our official; do
  if [ "$c" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
  "$BIN" -n "$W/ref_simple.net.xml" -r "$W/routes.rou" --no-step-log true \
    --vehroute-output "$OUT/w.$c.rou.xml" $WFLAGS >"$OUT/w.$c.out" 2>"$OUT/w.$c.err"
  echo "$c exit=$?"
  grep -E '<route|Error|Warning' "$OUT/w.$c.err" 2>/dev/null | head -10
  grep -E '<vehroute|<route' "$OUT/w.$c.rou.xml" 2>/dev/null | head -10
done
echo "=== AGENT RUN DONE ==="
