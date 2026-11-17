#!/bin/bash
# op02 scratch runner: committed-reference output rows for the person/FCD port. Removed before merge.
BIN=$CARGO_TARGET_DIR/debug/sumo bash tools/oracle/run_outputs.sh persons_nonint
