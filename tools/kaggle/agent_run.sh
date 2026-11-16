#!/usr/bin/env bash
# Branch-only (agent tc03): TraCI effect probes for changeLane timing, getVersion and wrong-length setRedYellowGreenState.
# Ours = /tmp/target/debug/sumo, C++ = official wheel binary in $SUMO_PRISTINE_BIN. Removed before the final commit.
export PYTHONPATH=/tmp/sumo_pkg
echo "pristine: $SUMO_PRISTINE_BIN"; ls "$SUMO_PRISTINE_BIN" | head -3
timeout 1200 python tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo --port 11700 \
  --filter "getVersion,changeLane,wrong len" 2>&1 | tail -150
