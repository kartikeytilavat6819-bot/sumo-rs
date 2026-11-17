#!/bin/bash
# op01 temporary: E3 output row vs committed official-C++ reference. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export BIN=$CARGO_TARGET_DIR/debug/sumo
export OUT=/tmp/op01out
export REFS=committed
export ONLY_OUT=e3
bash tools/oracle/run_outputs.sh stress
cat $OUT/results.tsv
