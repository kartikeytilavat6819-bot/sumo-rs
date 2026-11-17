#!/usr/bin/env bash
# TEMPORARY (tr04): TraCI vehicle/TL setter probes on the VM. Remove in the final commit.
set -u
VENV=/tmp/tr04venv
[ -x "$VENV/bin/python" ] || { python3 -m venv "$VENV" && "$VENV/bin/pip" install -q traci; }
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
i=0
for c in signal junction; do
  "$VENV/bin/python" tools/oracle/cases/traci_basic/client.py --case $c --ours "$OURS" --port $((9900 + i * 400)) --filter "dom " 2>&1 | grep -E "MATCH|DIFF|match|TOTAL|Error|error" | tail -n 400 &
  i=$((i + 1))
done
wait
echo "AGENT RUN DONE"
