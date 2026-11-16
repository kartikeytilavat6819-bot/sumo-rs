#!/bin/bash
# TEMPORARY agent run (measurement). Removed before the final commit.
bash tools/oracle/cases/state_roundtrip/roundtrip.sh /tmp/target/debug /tmp/rt 2>&1 | grep -E 'RESULT|state lines|mismatches|exit='
echo "--- junction state diff"
head -12 /tmp/rt/junction/state_vs_cpp.txt
echo "--- signal state diff"
head -12 /tmp/rt/signal/state_vs_cpp.txt
echo "AGENT RUN DONE"
