#!/bin/bash
set -u
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
export COMPLEX_FILTER="${COMPLEX_FILTER:-person/remove_crossing}"
export COMPLEX_LIMIT="${COMPLEX_LIMIT:-1}"
python3 tools/oracle/complex_suite.py "$@" 2>&1 | tail -5
echo "=====OFFICIAL tripinfo====="
find /tmp/complex_oracle -path "*person/remove_crossing/official*" -name tripinfo.xml -exec cat {} \;
echo "=====OURS tripinfo====="
find /tmp/complex_oracle -path "*person/remove_crossing/ours*" -name tripinfo.xml -exec cat {} \;