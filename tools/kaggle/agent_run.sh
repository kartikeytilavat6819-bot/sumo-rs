#!/bin/bash
# oc61 probe: full panic for the last failing striping case
set -u
cd /tmp/src || exit 1
C=tools/oracle/cases/tt_pedestrian_model_striping_improper_walk_via_bus_stop_access
FLAGS=$(cat $C/sim.args)
export RUST_BACKTRACE=0
echo "=== our binary ==="
timeout 120 /tmp/target/debug/sumo -n $C/ref_simple.net.xml -r $C/routes.rou $FLAGS \
  --tripinfo-output /tmp/oc61_ti.xml --fcd-output /tmp/oc61_fcd.xml --statistic-output /tmp/oc61_stat.xml \
  --no-step-log true 2>&1 | grep -A4 -m1 -E 'panicked at|Error:' | head -12
echo "rc=${PIPESTATUS[0]}"
