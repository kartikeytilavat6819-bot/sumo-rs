#!/bin/bash
# TEMPORARY (agent h06): run the new h06 TraCI probes, C++ vs ours. Removed in the final commit.
set -u
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
cd tools/oracle/cases/traci_basic
echo "== h06 junction (C++ 1.28.0 vs ours)"
timeout 900 python3 client.py --case junction --filter "h06 " --skip "h06 tl" --port 10800 --ours "$CARGO_TARGET_DIR/debug/sumo" 2>&1 | grep -v "^ours:" | tail -n 150
echo "== h06 signal TL probes"
timeout 300 python3 client.py --case signal --filter "h06 tl" --port 10900 --ours "$CARGO_TARGET_DIR/debug/sumo" 2>&1 | grep -v "^ours:" | tail -n 40
