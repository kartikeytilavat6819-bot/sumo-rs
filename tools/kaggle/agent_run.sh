#!/bin/bash
# du02 scratch runner: runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter >/dev/null 2>&1; echo "build rc=$?"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
cat "$OUT/results.tsv" | tr -d '\r'
