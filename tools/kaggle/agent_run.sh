#!/bin/bash
# oc61 probe: full panic for the last failing striping case
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
cd "$ROOT" || { echo "NO ROOT"; exit 1; }
echo "ROOT=$ROOT"
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
if [ ! -x "$BIN" ]; then
  BIN=$(ls -d /tmp/target*/debug/sumo /workspace*/target/debug/sumo 2>/dev/null | head -1)
fi
echo "BIN=$BIN"
C=tools/oracle/cases/tt_pedestrian_model_striping_improper_walk_via_bus_stop_access
FLAGS=$(cat $C/sim.args)
export RUST_BACKTRACE=0
echo "=== our binary ==="
timeout 120 "$BIN" -n $C/ref_simple.net.xml -r $C/routes.rou $FLAGS \
  --tripinfo-output /tmp/oc61_ti.xml --fcd-output /tmp/oc61_fcd.xml --statistic-output /tmp/oc61_stat.xml \
  --no-step-log true 2>&1 | tail -30
echo "rc=${PIPESTATUS[0]}"
