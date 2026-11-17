#!/bin/bash
# nx02 netconvert option sweep vs official 1.28.0 (TEMPORARY: removed in the final commit)
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export RUST_NETCONVERT=$CARGO_TARGET_DIR/debug/netconvert
export CPP_NETCONVERT=$SUMO_PRISTINE_BIN/netconvert
export CPP_SUMO_HOME=$SUMO_HOME
export NETOPTS_OUT=/tmp/netopts_nx02
export NETOPTS_JOBS=8
rm -rf "$NETOPTS_OUT"
bash tools/oracle/run_netopts.sh
