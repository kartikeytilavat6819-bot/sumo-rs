#!/bin/bash
# op02 scratch runner: committed-reference output rows for the person/FCD/emissions port. Removed before merge.
export OUT=/tmp/op02_out
export BIN=/tmp/target/debug/sumo
mkdir -p "$OUT"
bash tools/oracle/run_outputs.sh persons_nonint
ONLY_OUT=edgedata_emissions bash tools/oracle/run_outputs.sh real
