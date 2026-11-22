#!/bin/bash
# oc103 diagnostic: our sumo vs official sumo on an astar-landmark TAZ case.
set -u
OUT=/tmp/oc103
mkdir -p "$OUT"
C=tools/oracle/cases/tt_devices_rerouting_parallel_taz_astar_landmark_parallel
COMMON="-n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $OUT/ours.ti.xml --fcd-output $OUT/ours.fcd.xml --statistic-output $OUT/ours.stat.xml --no-step-log true"
EXTRA=$(cat "$C/sim.args")

echo "=== OURS ==="
timeout 120 "$CARGO_TARGET_DIR/debug/sumo" $COMMON $EXTRA >"$OUT/ours.log" 2>&1
echo "ours exit=$?"
head -40 "$OUT/ours.log"

echo "=== OFFICIAL ==="
timeout 120 "$SUMO_PRISTINE_BIN/sumo" $COMMON $EXTRA >"$OUT/ref.log" 2>&1
echo "ref exit=$?"
head -40 "$OUT/ref.log"

echo "=== lookuptable files nearby ==="
find tools/oracle/cases -maxdepth 2 -name 'lookuptable*' -print | head -20
