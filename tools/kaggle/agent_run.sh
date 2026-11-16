#!/usr/bin/env bash
# Branch-only (agent tc03): full TraCI probe harness. Removed before the final commit.
echo "pristine: $SUMO_PRISTINE_BIN"; ls "$SUMO_PRISTINE_BIN" | head -3
if [ ! -d "$SUMO_HOME/tools/traci" ]; then python -m pip install -q --target /tmp/traci_pkg traci==1.28.0 2>&1 | tail -2; fi
export PYTHONPATH=$SUMO_HOME/tools:/tmp/traci_pkg:/tmp/sumo_pkg
H="python tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo"
echo "=== run 1: effect probes"
timeout 600 $H --port 11700 --filter "effect:" 2>&1 | grep -E "DIFF|match|TOTAL|Traceback|Error"
echo "=== run 2: all other probes, 4 parallel shards"
for i in 0 1 2 3; do
  (timeout 780 $H --port $((12700 + i * 1000)) --skip "effect:" --shard $i/4 2>&1 | grep -E "DIFF|match|TOTAL|Traceback|Error" > /tmp/tc03_shard$i.txt) &
done
wait
for i in 0 1 2 3; do echo "--- shard $i"; cat /tmp/tc03_shard$i.txt; done
