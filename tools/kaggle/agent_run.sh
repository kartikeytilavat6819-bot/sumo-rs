#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
echo "=== od2trips ==="
bash tools/oracle/run_odtool.sh od2trips 2>&1 | tail -5
echo "failing:"
grep -E 'FAIL|ERROR|PANIC|REFERR' /tmp/od2trips_oracle/results.tsv | head -40