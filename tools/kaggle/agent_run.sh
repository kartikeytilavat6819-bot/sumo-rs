#!/bin/bash
# oc93 probe 6: env-gated counters (OC93_COUNT=1) printed to stderr by our binary.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUR="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc93
mkdir -p "$OUT"
c=tt_rail_siding_loop_3veh
dir="tools/oracle/cases/$c"
args=$(tr -d '\r' < "$dir/sim.args")
OC93_COUNT=1 timeout 30 "$OUR" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
    --fcd-output "$OUT/$c.fcd.xml" --no-step-log true $args \
    > "$OUT/$c.out" 2> "$OUT/$c.err"
echo "exit=$?"
echo "==== first 3 and last 5 OC93 counter lines ===="
grep -a "^OC93" "$OUT/$c.err" | sed -n "1,3p"; grep -a "^OC93" "$OUT/$c.err" | tail -5
echo "==== last stdout lines ===="
tail -5 "$OUT/$c.out"
echo "last fcd: $(grep -a '<timestep' "$OUT/$c.fcd.xml" 2>/dev/null | tail -1 | grep -ao 'time="[0-9.]*"' | tail -1)"
echo "AGENT RUN DONE"
