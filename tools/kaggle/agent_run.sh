#!/bin/bash
set -u
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python3 tools/oracle/complex_suite.py "$@" 2>&1 | grep -E "FAIL |complex suite oracle" | head -100