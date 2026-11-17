#!/bin/bash
# st04 temporary: compare the junction rerouteTraveltime-between-stops probe with C++. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python3 tools/oracle/cases/traci_basic/client.py --port 9700 --ours "$CARGO_TARGET_DIR/debug/sumo" \
    --case junction --filter "rerouteTraveltime(v0) between" 2>/dev/null | tail -n 6
