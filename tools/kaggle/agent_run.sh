#!/bin/bash
# temporary (sl05): state/load harness on the VM
cd "$(dirname "$0")/../.." || exit 1
bash tools/oracle/run_state.sh 2>&1 | tail -260
echo "AGENT RUN DONE"
