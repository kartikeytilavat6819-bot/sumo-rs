#!/bin/bash
# Round 1: probe the gate environment, then measure the whole TraCI suite.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
echo "=== env probe ==="
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "SUMO_HOME=$SUMO_HOME"
echo "SUMO_PRISTINE_BIN=$SUMO_PRISTINE_BIN"
for f in "$CARGO_TARGET_DIR/debug/sumo" "$SUMO_PRISTINE_BIN/sumo" "$SUMO_PRISTINE_BIN/TraCITestClient"; do
  if [ -x "$f" ]; then echo "OK $f"; else echo "MISSING $f"; fi
done
echo "tests in SUMO_HOME:"
ls "$SUMO_HOME/tests/traci" 2>/dev/null | head -5 || true
echo "imported tests:"
ls tools/oracle/traci_inputs 2>/dev/null | head -5 || true
echo "python3: $(command -v python3 || echo none)"
echo "=== measurement ==="
export TRACI_JOBS=${TRACI_JOBS:-6}
export TRACI_SHOW=${TRACI_SHOW:-70}
export TRACI_SUITE_OUT=/tmp/traci_oc64
timeout 1500 bash tools/oracle/run_traci_suite.sh 2>&1 | tail -350
echo "=== result table head ==="
head -5 "$TRACI_SUITE_OUT/results.tsv" 2>/dev/null || true
