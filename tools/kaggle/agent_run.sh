#!/bin/bash
# oc52 differential router fuzzing: run the fuzzer on the gate VM and print concise results plus
# every failing case's inputs as base64 so it can be rebuilt as a permanent oracle case.
cd "$(dirname "$0")/../.." || exit 1
export FUZZ_OUT=/tmp/fuzz_router_oc52
export FUZZ_BUDGET=900
bash tools/oracle/fuzz_router.sh 20 2>&1
echo "AGENT FUZZ DONE"
