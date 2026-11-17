#!/bin/bash
# tx01 trace: what the TX01 prints captured (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$OUT/ours.fcd.xml" \
  --statistic-output "$OUT/ours.stat.xml" --no-step-log true $(cat sim.args) >"$OUT/ours.log" 2>&1
echo "== exit=$?"
echo "TX01 lines: $(grep -c TX01 "$OUT/ours.log")"
grep TX01 "$OUT/ours.log" | head -3
echo "-- taxi0 update/move lines with t in 160..180 (every 4th)"
grep -E 'TX01 (upd|move) t=[0-9]+ veh=taxi0' "$OUT/ours.log" | awk -F'[ =]' '$4>=160 && $4<=180' | head -60
echo "-- idle lines t in 100..180"
grep 'TX01 idle' "$OUT/ours.log" | awk -F'[ =]' '$4>=100 && $4<=180' | head -40
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-220
