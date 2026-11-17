#!/bin/bash
# pv03 temporary: TraCI probes for actuated + NEMA parameters and program switching. Removed in the final commit.
export SUMO_HOME="${SUMO_HOME:-$HOME/sumo-pristine/lib/python3.12/site-packages/sumo}"
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
export SUMO_PRISTINE_BIN="${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}"
OURS="$CARGO_TARGET_DIR/debug/sumo"
for c in signal_actuated traci_nema; do
  echo "=== PROBE $c"
  timeout 900 python3 tools/oracle/cases/traci_basic/client.py --case "$c" --ours "$OURS" --port 9800 2>&1 | tail -n 150
  echo "=== EXIT $c $?"
done
