#!/bin/bash
# oc108: probe optimizeStops stop replacement (Rust vs official C++).
set -u
OUT=/tmp/oc108
mkdir -p "$OUT"
C=tools/oracle/cases/tt_extended_rerouter_optimizeStops_grid5x10
FLAGS="--xml-validation never --write-license --default.speeddev 0 --default.departspeed 0 --default.departlane first -v --device.rerouting.mode 8 -a $C/input_additional.add.xml,$C/input_additional2.add.xml --no-step-log true"
echo "=== RUST ==="
${CARGO_TARGET_DIR}/debug/sumo -n "$C/ref_simple.net.xml" -r "$C/routes.rou" $FLAGS \
    --tripinfo-output "$OUT/rust.tripinfo.xml" >"$OUT/rust.log" 2>&1 || echo "rust exit $?"
grep -E 'OC108_OPTI|OC108_REPL|OC108_REROUTE|skip|replac|could not assign|Error' "$OUT/rust.log" | head -160
echo "=== CPP ==="
${SUMO_PRISTINE_BIN}/sumo -n "$C/ref_simple.net.xml" -r "$C/routes.rou" $FLAGS \
    --tripinfo-output "$OUT/cpp.tripinfo.xml" >"$OUT/cpp.log" 2>&1 || echo "cpp exit $?"
grep -E 'skip|replac|could not assign|Error' "$OUT/cpp.log" | head -60
echo "=== ARRIVALS ==="
grep -o 'arrival="[^"]*"' "$OUT/rust.tripinfo.xml" | head -3
grep -o 'arrival="[^"]*"' "$OUT/cpp.tripinfo.xml" | head -3
