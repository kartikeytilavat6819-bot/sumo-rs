#!/bin/bash
# Targeted: full failure map + raw stderr of one error test.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export TRACI_JOBS=12 TRACI_KILL_AFTER=20 TRACI_SHOW=0
export TRACI_SUITE_OUT=/tmp/traci_oc64
bash tools/oracle/run_traci_suite.sh 2>&1 | tail -4
echo "=== one error-test raw outputs ==="
find "$TRACI_SUITE_OUT/work" -path '*Edge/errors/UnknownID*' -type f -name 'stderr_*' | while read f; do
  echo "FILE $f"; cat "$f"; done
echo "=== diffs head ==="
head -60 "$TRACI_SUITE_OUT/diffs.txt" 2>/dev/null || true
echo "AGENT RUN DONE"
