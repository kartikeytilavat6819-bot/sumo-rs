#!/bin/bash
set -u
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
cd tools/oracle/cases/traci_basic
for c in junction signal; do
echo "== all probes case $c"
timeout 800 python3 client.py --case $c --port 9300 --ours "$CARGO_TARGET_DIR/debug/sumo" 2>&1 | grep -v "^ours:" | tail -n 150
done
