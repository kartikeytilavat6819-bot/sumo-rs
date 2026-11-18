#!/bin/bash
# Run the official TraCI test-client suite against our sumo and the pristine C++ sumo.
# Environment: TRACI_LIMIT, TRACI_ONLY, TRACI_JOBS, TRACI_SUITE_OUT, TRACI_TESTS.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
export PATH="$HOME/.cargo/bin:$PATH"
exec python3 "$HERE/run_traci_suite.py"
