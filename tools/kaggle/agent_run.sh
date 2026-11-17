#!/usr/bin/env bash
# TEMPORARY (tr03): TraCI probes on Kaggle/VM. Remove in the final commit.
set -u
python3 -c "import traci" 2>/dev/null || pip install -q traci
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
python3 tools/oracle/cases/traci_basic/client.py --ours "${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo" --port 9600 --filter highlight 2>&1 | grep -E "MATCH|DIFF|match|TOTAL|Error|error" | tail -n 150
