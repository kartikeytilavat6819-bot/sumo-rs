#!/bin/bash
# Temporary (sl03): state/load parity for every sim case. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/state_sl03 JOBS=16
bash tools/oracle/run_state.sh 2>&1 | grep -vE '^$' | tail -150
