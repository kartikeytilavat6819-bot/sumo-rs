#!/bin/bash
# st04 temporary run: TraCI replaceStop / insertStop / rerouteTraveltime probes at base port 9700. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python3 tools/oracle/cases/traci_basic/client.py --port 9700 --ours "$CARGO_TARGET_DIR/debug/sumo" \
    --filter "replaceStop,insertStop,rerouteTraveltime,setStop(v0)" 2>&1 | tail -n 120
