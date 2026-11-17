#!/bin/bash
# tx01 trace: taxi0 parking-stop state (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$OUT/ours.fcd.xml" \
  --statistic-output "$OUT/ours.stat.xml" --no-step-log true $(cat sim.args) >"$OUT/ours.log" 2>&1
echo "== exit=$?"
echo "-- taxi0 parking checks"
grep 'TX01 park' "$OUT/ours.log" | awk '{print}' | head -90
echo "-- removal/enter"
grep -E 'TX01 (removal|enter) veh=taxi0' "$OUT/ours.log" | head -10
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-200
