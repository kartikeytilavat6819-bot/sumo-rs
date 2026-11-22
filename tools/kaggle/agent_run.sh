#!/bin/bash
# oc103 diagnostic: our sumo vs official/comitted refs for two failing cases.
set -u
OUT=/tmp/oc103
mkdir -p "$OUT"

# --- AStar landmark error text (already diagnosed; kept for regression) ---
C=tools/oracle/cases/tt_devices_rerouting_parallel_taz_astar_landmark_parallel
COMMON="-n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $OUT/ours.ti.xml --fcd-output $OUT/ours.fcd.xml --statistic-output $OUT/ours.stat.xml --no-step-log true"
EXTRA=$(cat "$C/sim.args")
echo "=== ASTAR OURS ==="
timeout 120 "$CARGO_TARGET_DIR/debug/sumo" $COMMON $EXTRA >"$OUT/astar.ours.log" 2>&1
echo "ours exit=$?"; grep -E 'Error:|Quitting' "$OUT/astar.ours.log" | head -5
echo "=== ASTAR OFFICIAL ==="
timeout 120 "$SUMO_PRISTINE_BIN/sumo" $COMMON $EXTRA >"$OUT/astar.ref.log" 2>&1
echo "ref exit=$?"; grep -E 'Error:|Quitting' "$OUT/astar.ref.log" | head -5

# --- Taxi idling randomCircling: compare our tripinfo/fcd to committed refs ---
T=tools/oracle/cases/tt_devices_taxi_idling_randomCircling_rerouting
TEXTRA=$(cat "$T/sim.args")
echo "=== TAXI OURS ==="
timeout 300 "$CARGO_TARGET_DIR/debug/sumo" -n "$T/ref_simple.net.xml" -r "$T/routes.rou" \
  --tripinfo-output "$OUT/taxi.ti.xml" --fcd-output "$OUT/taxi.fcd.xml" \
  --statistic-output "$OUT/taxi.stat.xml" --no-step-log true $TEXTRA >"$OUT/taxi.log" 2>&1
echo "taxi exit=$?"; grep -E 'Error:|Warning:|Quitting' "$OUT/taxi.log" | head -20
echo "--- tripinfo diff ---"
python tools/oracle/compare_sim.py --tripinfo "$T/ref.tripinfo.xml" "$OUT/taxi.ti.xml" 2>&1 | head -30
echo "--- fcd diff ---"
python tools/oracle/compare_sim.py --fcd "$T/ref.fcd.xml" "$OUT/taxi.fcd.xml" 2>&1 | head -30
echo "--- stat diff ---"
python tools/oracle/compare_stat.py "$T/ref.stat.xml" "$OUT/taxi.stat.xml" 2>&1 | head -20

echo "=== AGENT RUN DONE ==="
