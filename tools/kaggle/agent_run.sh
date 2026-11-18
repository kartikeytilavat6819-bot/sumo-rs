#!/bin/bash
# Gate diagnostic for oc58: get full panic message for one opposite-direction PANIC row.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-panic
rm -rf "$OUT"; mkdir -p "$OUT"
for c in tt_sublane_model_opposite_direction_driving_bugs_ticket8889 tt_sublane_model_opposite_direction_driving_stop_opposite_2edges; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  echo "=== $c ==="
  RUST_BACKTRACE=1 timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  echo "rc=$?"
  grep -m1 -A2 -E 'panicked at|thread.*panicked' "$OUT/$c.log"
  echo "--- backtrace frames (sumo_core) ---"
  grep -E 'sumo_core::microsim::ms_lane_changer_sublane|ms_lcm' "$OUT/$c.log" | head -12
done
