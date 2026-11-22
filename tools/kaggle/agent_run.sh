#!/bin/bash
# oc102: dump the ego's junction-decision state at t=25-27 for 3lanes_left.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc102
mkdir -p "$OUT"
CASE=tools/oracle/cases/tt_devices_bluelight_3lanes_left
BIN="${CARGO_TARGET_DIR:?}/debug/sumo"
if [ ! -x "$BIN" ] && [ -x "$BIN.exe" ]; then BIN="$BIN.exe"; fi
FLAGS=$(tr -d '\r' < "$CASE/sim.args")
OC102_DEBUG=1 timeout 300 "$BIN" -n "$CASE/ref_simple.net.xml" -r "$CASE/routes.rou" \
    --fcd-output "$OUT/fcd.xml" --no-step-log true $FLAGS >"$OUT/log" 2>&1
echo "rc=$?"
grep 'OC102' "$OUT/log" || true
