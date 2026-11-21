#!/bin/bash
# Measure the no_permitted_route_lane TraCI test on both binaries.
set -u
TEST=tools/oracle/complex_bundle/vehicle/moveToXY/no_permitted_route_lane
NC="${SUMO_PRISTINE_BIN}/netconvert"
[ -x "$NC" ] || NC="${CARGO_TARGET_DIR}/debug/netconvert"
for side in official ours; do
  d="/tmp/oc100_$side"
  rm -rf "$d"
  cp -r "$TEST" "$d"
  if [ "$side" = official ]; then bin="${SUMO_PRISTINE_BIN}/sumo"; else bin="${CARGO_TARGET_DIR}/debug/sumo"; fi
  (cd "$d" && SUMO_BINARY="$bin" NETCONVERT_BINARY="$NC" PYTHONPATH="${SUMO_HOME}/tools:${PYTHONPATH:-}" python3 runner.py > console.txt 2>&1)
  echo "===== $side console ====="
  cat "$d/console.txt"
done
