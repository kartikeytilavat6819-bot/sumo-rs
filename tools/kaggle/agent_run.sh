#!/usr/bin/env bash
# TEMPORARY (tr03): TraCI probes on Kaggle/VM. Remove in the final commit.
set -u
VENV=/tmp/tr03venv
[ -x "$VENV/bin/python" ] || { python3 -m venv "$VENV" && "$VENV/bin/pip" install -q traci; }
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
for c in ${PROBE_CASES:-junction signal}; do
  "$VENV/bin/python" tools/oracle/cases/traci_basic/client.py --case $c --ours "$OURS" --port 9600 --filter "${PROBE_FILTER:-dom }" 2>&1 | grep -E "MATCH|DIFF|match|TOTAL|Error|error" | tail -n 400
done
echo "AGENT RUN DONE"
