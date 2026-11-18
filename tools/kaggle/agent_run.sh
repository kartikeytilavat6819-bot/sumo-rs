#!/bin/bash
# oc16 measurement: full panic + backtrace for lc_lc2013_turnlanes_dur/ref
C=tools/oracle/cases/lc_lc2013_turnlanes_dur
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
OUT=${OUT:-/tmp/oc16_run}
mkdir -p "$OUT"
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
echo "== running our sumo: $BIN"
RUST_BACKTRACE=1 "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
    --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
    --statistic-output "$OUT/stat.xml" --no-step-log true \
    --end 150 --lanechange.duration 2 >"$OUT/sumo.log" 2>&1
echo "exit=$?"
echo "== last 40 lines of sumo.log"
tail -40 "$OUT/sumo.log"
