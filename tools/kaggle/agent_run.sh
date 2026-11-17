#!/usr/bin/env bash
# Branch-only (agent tc03): full TraCI probe harness. Removed before the final commit.
echo "pristine: $SUMO_PRISTINE_BIN"; ls "$SUMO_PRISTINE_BIN" | head -3
if [ ! -d "$SUMO_HOME/tools/traci" ]; then python -m pip install -q --target /tmp/traci_pkg traci==1.28.0 2>&1 | tail -2; fi
export PYTHONPATH=$SUMO_HOME/tools:/tmp/traci_pkg:/tmp/sumo_pkg
H="python tools/oracle/cases/traci_basic/client.py --ours ${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
N=8
for s in cpp ours; do timeout 120 $H --only $s --port 9100 --filter getLaneID 2>&1 | tail -4; tail -12 traci_${s}_*.log; done
for i in $(seq 0 $((N - 1))); do
  (timeout 540 $H --port $((9200 + i * 200)) --shard $i/$N 2>&1 | grep -E "DIFF|match|TOTAL|Traceback|Error" > /tmp/tc03_shard$i.txt) &
done
wait
for i in $(seq 0 $((N - 1))); do echo "--- shard $i"; cat /tmp/tc03_shard$i.txt; done
