#!/bin/bash
# temporary (sl05): run the state/load harness on the VM and print its per-variant rows
cd "$(dirname "$0")/../.." || exit 1
bash tools/oracle/run_state.sh 2>&1 | tail -260
