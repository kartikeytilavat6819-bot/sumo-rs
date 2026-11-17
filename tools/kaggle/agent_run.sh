#!/bin/bash
# tx01 trace: taxi0 stop-end window 166..177 (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$OUT/ours.fcd.xml" \
  --statistic-output "$OUT/ours.stat.xml" --no-step-log true $(cat sim.args) >"$OUT/ours.log" 2>&1
echo "== exit=$?"
grep -E 'TX01 .* t=(16[6-9]|17[0-7]) ' "$OUT/ours.log" | grep 'taxi0\|veh=taxi0\|idle' | head -80
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-220
