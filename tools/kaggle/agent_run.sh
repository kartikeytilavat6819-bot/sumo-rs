#!/bin/bash
# op02 scratch runner: committed-reference output rows for the person/FCD/emissions port. Removed before merge.
export BIN=$CARGO_TARGET_DIR/debug/sumo
bash tools/oracle/run_outputs.sh persons_nonint
ONLY_OUT=edgedata_emissions bash tools/oracle/run_outputs.sh real
