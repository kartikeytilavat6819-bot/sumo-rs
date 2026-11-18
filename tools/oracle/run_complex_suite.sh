#!/bin/bash
# Compare official SUMO and our port on the bundled tests/complex/traci runner.py scripts.
# Usage: COMPLEX_FILTER=vehicle COMPLEX_LIMIT=20 bash tools/oracle/run_complex_suite.sh
set -u
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python3 tools/oracle/complex_suite.py "$@"
