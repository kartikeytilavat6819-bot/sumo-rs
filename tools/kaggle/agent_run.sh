#!/bin/bash
# oc44 differential fuzz measurement (branch-only; removed before final commit).
echo "AGENT_RUN_START $(date +%s)"
FUZZ_FIRST=${FUZZ_FIRST:-7000} FUZZ_BUDGET=${FUZZ_BUDGET:-30} bash tools/oracle/fuzz_sim.sh 5
echo "AGENT_RUN_END $(date +%s)"
