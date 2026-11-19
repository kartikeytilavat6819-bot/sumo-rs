#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
echo "=== jtrrouter ==="
bash tools/oracle/run_jtrrouter.sh 2>&1 | tail -3
grep -iE 'FAIL|ERROR|PANIC|REFERR' /tmp/jtrrouter_oracle/results.tsv | head
echo "=== marouter ==="
bash tools/oracle/run_odtool.sh marouter 2>&1 | tail -2
grep -iE 'FAIL|ERROR|PANIC|REFERR' /tmp/marouter_oracle/results.tsv | head