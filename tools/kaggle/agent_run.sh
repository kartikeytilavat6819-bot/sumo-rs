#!/bin/bash
# Temporary audit probe: print failing rows from the oracle gate.
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|TIMEOUT|PASS='
echo "--- depart variants ---"
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|/15'
