#!/bin/bash
# TEMPORARY agent run (trace). Removed before the final commit.
bash tools/oracle/cases/state_roundtrip/roundtrip.sh /tmp/target/debug /tmp/rt 2>&1 | tail -8
echo "--- TRACE junc-graph (internal edges)"
grep -h "TRACE junc-graph" /tmp/rt/junction/save.log /tmp/rt/signal/save.log | head -20
echo "--- TRACE save-lane"
grep -h "TRACE save-lane" /tmp/rt/junction/save.log /tmp/rt/signal/save.log | head -30
echo "--- state diff (signal)"
head -40 /tmp/rt/signal/state_vs_cpp.txt
