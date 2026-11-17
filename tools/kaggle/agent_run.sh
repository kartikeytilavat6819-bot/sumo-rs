#!/bin/bash
# op01 temporary: output rows for the real case vs committed official-C++ references. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export BIN=$CARGO_TARGET_DIR/debug/sumo
export OUT=/tmp/op01out
export REFS=committed
bash tools/oracle/run_outputs.sh real
cat $OUT/results.tsv
