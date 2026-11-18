#!/bin/bash
# oc65 probe 2: locate usable traci + complex tests on the gate machine (temporary).
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
echo "=== TRACI VIA SUMO_HOME/tools"
python3 -c 'import traci, sumolib; print("traci", traci.__file__); print("sumolib", sumolib.__file__)' 2>&1 | head -8
echo "=== data/tests TREE (2 levels)"
find "$SUMO_HOME/data/tests" -maxdepth 2 -type d 2>/dev/null | head -40
echo "=== data/tests FILE COUNT"
find "$SUMO_HOME/data/tests" -type f 2>/dev/null | wc -l
echo "=== runner.py ANYWHERE"
find "$SUMO_HOME" -type f -name runner.py 2>/dev/null | head -10
echo "=== tools dir sample"
ls "$SUMO_HOME/tools" | head -25
echo "=== traci dir"
ls "$SUMO_HOME/tools/traci" 2>&1 | head -25
echo "=== version"
"$SUMO_PRISTINE_BIN/sumo" --version 2>&1 | head -2
"$CARGO_TARGET_DIR/debug/sumo" --version 2>&1 | head -2
echo "=== DONE PROBE2"
