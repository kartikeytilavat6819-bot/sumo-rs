#!/bin/bash
# Gate diagnostic for oc58: capture the current panic location for opposite-direction rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-panic2
rm -rf "$OUT"; mkdir -p "$OUT"
for c in tt_sublane_model_opposite_direction_driving_bugs_ticket8889 tt_sublane_model_opposite_direction_driving_stop_opposite_2edges tt_sublane_model_opposite_direction_driving_overtaking_stopped_laneGain; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  echo "=== $c ==="
  RUST_BACKTRACE=1 timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  echo "rc=$?"
  grep -m1 -A1 -E 'panicked at' "$OUT/$c.log"
  echo '--- frames ---'
  grep -E 'sumo_core::microsim' "$OUT/$c.log" | head -10
done
