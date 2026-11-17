#!/bin/bash
# du02 scratch runner: builds duarouter and runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter >/dev/null 2>&1; echo "build rc=$?"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
echo "--- trips cand.log DU02 (first 40)"; grep -a 'DU02' "$OUT/duarouter_trips/cand.log" | head -40
echo "--- trips cand.log tail"; tail -n 12 "$OUT/duarouter_trips/cand.log" | tr -d '\r'
