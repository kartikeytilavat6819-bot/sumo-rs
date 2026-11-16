#!/usr/bin/env bash
# Branch-only (agent tc03): TraCI effect probes for changeLane timing, getVersion and wrong-length setRedYellowGreenState.
# Ours = /tmp/target/debug/sumo, C++ = official wheel binary in $SUMO_PRISTINE_BIN. Removed before the final commit.
echo "pristine: $SUMO_PRISTINE_BIN"; ls "$SUMO_PRISTINE_BIN" | head -3
if [ ! -d "$SUMO_HOME/tools/traci" ]; then python -m pip install -q --target /tmp/traci_pkg traci==1.28.0 2>&1 | tail -2; fi
export PYTHONPATH=$SUMO_HOME/tools:/tmp/traci_pkg:/tmp/sumo_pkg
python -c 'import traci,os;print("traci:", os.path.dirname(traci.__file__))' 2>&1 | tail -1
timeout 1200 python tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo --port 11700 \
  --filter "getVersion,changeLane,wrong len" 2>&1 | tail -150
