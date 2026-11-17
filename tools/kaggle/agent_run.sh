#!/bin/bash
# om01 city output parity (both binaries live on the VM). Removed in the final commit.
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export OUT=/tmp/city_outputs
bash tools/oracle/run_outputs_city.sh ${CASES:-}
