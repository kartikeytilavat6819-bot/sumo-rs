#!/bin/bash
# tx01 trace: taxi0 park/unpark events (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$OUT/ours.fcd.xml" \
  --statistic-output "$OUT/ours.stat.xml" --no-step-log true $(cat sim.args) >"$OUT/ours.log" 2>&1
echo "== exit=$?"
echo "-- taxi0 removal/enter events"
grep -E 'TX01 (removal|enter) veh=taxi0' "$OUT/ours.log" | head -40
echo "-- idle events"
grep 'TX01 idle' "$OUT/ours.log" | head -20
echo "-- taxi0 move/upd t in 100000..176000"
grep -E 'TX01 (move|upd) t=[0-9]+ veh=taxi0' "$OUT/ours.log" | awk -F'[ =]' '$4>=100000 && $4<=176000' | head -30
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-200
