#!/bin/bash
set -u
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
export COMPLEX_FILTER="${COMPLEX_FILTER:-vehicle/moveToXY/no_permitted_route_lane}"
export COMPLEX_LIMIT="${COMPLEX_LIMIT:-1}"
python3 tools/oracle/complex_suite.py "$@" 2>&1 | tail -20
echo "=====DIFF====="
find /tmp/complex_oracle -name diff.txt -path "*${COMPLEX_FILTER}*" -exec cat {} \; 2>/dev/null | head -120