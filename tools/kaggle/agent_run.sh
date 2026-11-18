#!/bin/bash
# Measure the full TraCI suite with the Python raw-protocol driver.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
echo "=== quick driver sanity (one test) ==="
export TRACI_JOBS=4 TRACI_KILL_AFTER=20 TRACI_SHOW=20
export TRACI_SUITE_OUT=/tmp/traci_oc64
bash tools/oracle/run_traci_suite.sh 2>&1 | tail -30
echo "=== results table head ==="
head -20 "$TRACI_SUITE_OUT/results.tsv" 2>/dev/null || true
echo "AGENT RUN DONE"
