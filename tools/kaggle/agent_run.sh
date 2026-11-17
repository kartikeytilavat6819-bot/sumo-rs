#!/bin/bash
# du03 scratch runner: builds duarouter and runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter 2>&1 | grep -E '^(error|warning: unused)' | head -20; echo "build rc=${PIPESTATUS[0]}"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
cat "$OUT/results.tsv" | tr -d '\r'
