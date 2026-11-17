#!/usr/bin/env bash
# TEMPORARY (tr04): TraCI vehicle/TL setter probes on the VM. Remove in the final commit.
set -u
VENV=/tmp/tr04venv
if [ ! -x "$VENV/bin/python" ]; then
  python3 -m venv "$VENV" || echo "VENV CREATE FAILED"
  "$VENV/bin/pip" install -q traci 2>&1 | tail -n 5
fi
echo "venv python: $(ls -l $VENV/bin/python 2>&1)"
echo "traci: $("$VENV/bin/python" -c 'import traci; print(traci.__file__)' 2>&1 | tail -n 2)"
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
echo "ours: $(ls -l $OURS 2>&1)"
i=0
for c in signal junction; do
  "$VENV/bin/python" tools/oracle/cases/traci_basic/client.py --case $c --ours "$OURS" --port $((9900 + i * 400)) --filter "dom " > /tmp/tr04_$c.log 2>&1 &
  i=$((i + 1))
done
wait
for c in signal junction; do
  echo "== $c"
  grep -E "MATCH|DIFF|match|TOTAL" /tmp/tr04_$c.log | tail -n 400
  echo "-- raw tail $c"
  tail -n 15 /tmp/tr04_$c.log
done
echo "AGENT RUN DONE"
