#!/bin/bash
# oc52 differential router fuzzing. Keep this short: the gate pipes the whole output over ssh.
cd "$(dirname "$0")/../.." || exit 1
echo "oc52 agent run start"
export FUZZ_OUT=/tmp/fuzz_router_oc52
export FUZZ_BUDGET=300
export FUZZ_TMO=30
bash tools/oracle/fuzz_router.sh 16 2>&1
echo "AGENT FUZZ DONE"
