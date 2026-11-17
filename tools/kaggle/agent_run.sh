#!/bin/bash
# ou04 measurement runner (removed in the final commit; never reaches main)
export OUT=${OUT:-/tmp/ou04_out}
export BIN=$CARGO_TARGET_DIR/debug/sumo
bash tools/oracle/run_outputs.sh stops_transport stress 2>&1
echo "=== results.tsv ==="
cut -f1-3,6 "$OUT/results.tsv" | cut -c1-260
