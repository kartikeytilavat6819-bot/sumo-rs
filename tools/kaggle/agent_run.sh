#!/usr/bin/env bash
# Branch-only (agent tc03): full TraCI probe harness, split in two runs. Removed before the final commit.
echo "pristine: $SUMO_PRISTINE_BIN"; ls "$SUMO_PRISTINE_BIN" | head -3
if [ ! -d "$SUMO_HOME/tools/traci" ]; then python -m pip install -q --target /tmp/traci_pkg traci==1.28.0 2>&1 | tail -2; fi
export PYTHONPATH=$SUMO_HOME/tools:/tmp/traci_pkg:/tmp/sumo_pkg
echo "=== run 1: effect probes"
timeout 780 python tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo --port 11700 \
  --filter "effect:" 2>&1 | grep -E "DIFF|match|TOTAL|Traceback|Error"
echo "=== run 2: all other probes"
timeout 780 python tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo --port 12700 \
  --skip "effect:" 2>&1 | grep -E "DIFF|match|TOTAL|Traceback|Error"
