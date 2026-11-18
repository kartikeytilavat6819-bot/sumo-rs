#!/bin/bash
# Full TraCI suite measurement: summary + category breakdown + representative diffs.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export TRACI_JOBS=12 TRACI_KILL_AFTER=20 TRACI_SHOW=0
export TRACI_SUITE_OUT=/tmp/traci_oc64
bash tools/oracle/run_traci_suite.sh 2>&1 | tail -5
echo "=== diffs (first 220 lines) ==="
head -220 "$TRACI_SUITE_OUT/diffs.txt" 2>/dev/null || true
echo "AGENT RUN DONE"
