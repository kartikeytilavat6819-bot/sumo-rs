#!/usr/bin/env bash
# TEMPORARY (tr03): TraCI diagnostics on Kaggle/VM. Remove in the final commit.
set -u
VENV=/tmp/tr03venv
[ -x "$VENV/bin/python" ] || { python3 -m venv "$VENV" && "$VENV/bin/pip" install -q traci; }
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
for c in junction signal; do
  for side in cpp ours; do
    echo "== $c $side"
    "$VENV/bin/python" tools/oracle/cases/traci_basic/client.py --only $side --case $c --ours "$OURS" --port 9600 --filter diag 2>&1 | tail -n 40
  done
done
echo "AGENT RUN DONE"
