#!/usr/bin/env bash
# TEMPORARY (tr04): TraCI vehicle/TL setter probes on the VM. Remove in the final commit.
# The gate kills this script after 600 s, so the probes run as 4 parallel shards (2 cases x 2 halves).
set -u
VENV=/tmp/tr04venv
if [ ! -x "$VENV/bin/python" ]; then
  python3 -m venv "$VENV" || echo "VENV CREATE FAILED"
  "$VENV/bin/pip" install -q traci 2>&1 | tail -n 5
fi
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
i=0
for c in signal junction; do
  for s in 0 1; do
    "$VENV/bin/python" tools/oracle/cases/traci_basic/client.py --case $c --ours "$OURS" \
      --port $((9900 + i * 200)) --shard $s/2 --filter "dom " > /tmp/tr04_${c}_$s.log 2>&1 &
    i=$((i + 1))
  done
done
wait
for c in signal junction; do
  for s in 0 1; do
    echo "== $c shard $s"
    echo "MATCH rows: $(grep -c ' MATCH$' /tmp/tr04_${c}_$s.log)"
    grep -E " DIFF$|match$" /tmp/tr04_${c}_$s.log | cut -c1-260
    grep -aE "Traceback|Error:" /tmp/tr04_${c}_$s.log | tail -n 3
  done
done
echo "AGENT RUN DONE"
