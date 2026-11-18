#!/bin/bash
# oc65 probe of the gate VM environment (temporary; removed before final commit).
echo "=== ENV"
echo "SUMO_HOME=$SUMO_HOME"
echo "SUMO_PRISTINE_BIN=$SUMO_PRISTINE_BIN"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "=== PYTHON"
python3 -V
python3 -c 'import traci, sumolib; print("traci", traci.__file__); print("sumolib", sumolib.__file__)' 2>&1 | head -5
echo "=== BINARIES"
ls -l "$CARGO_TARGET_DIR/debug/sumo" "$SUMO_PRISTINE_BIN/sumo" 2>&1
echo "=== TESTS ON VM"
find /home/gate -maxdepth 6 -type d -name complex 2>/dev/null | head -20
find / -maxdepth 5 -type d -path '*/tests/complex' 2>/dev/null | head -20
echo "=== SUMO PACKAGE TREE"
ls "$SUMO_HOME" 2>&1 | head -30
echo "=== SUMO PACKAGE TESTS"
find "$SUMO_HOME" -maxdepth 3 -type d -name tests 2>/dev/null | head
echo "=== DONE PROBE"
