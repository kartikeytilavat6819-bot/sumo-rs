#!/bin/bash
# tx01 trace: taxi0 FCD every 5 s, ref vs ours (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output "$OUT/ref.ti.xml" --fcd-output "$OUT/ref.fcd.xml" \
  --statistic-output "$OUT/ref.stat.xml" --no-step-log true $(cat sim.args) >"$OUT/ref.log" 2>&1
echo "== exit=$?"
for f in ref.fcd.xml "$OUT/ref.fcd.xml"; do
  echo "-- $f"
  awk -F'"' '/<timestep/{t=$2} /<vehicle id="taxi0"/{ if (int(t)%5==0 && !seen[int(t)]++) print "t=" t, $0 }' "$f" | cut -c1-150 | head -90
done
