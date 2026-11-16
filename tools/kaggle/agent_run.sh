#!/usr/bin/env bash
# TEMPORARY (ls07): TraCI probes on Kaggle. Remove before merge.
set -u
python3 -c "import traci" 2>/dev/null || pip install -q traci
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
python3 tools/oracle/cases/traci_basic/client.py --ours /tmp/target/debug/sumo --port 12700 2>&1 | grep -E "MATCH|DIFF|match|TOTAL" | tail -n 150
