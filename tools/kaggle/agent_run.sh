#!/bin/bash
# du02 scratch runner: builds duarouter and runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter 2>&1 | grep -E '^(error|warning: unused)' -A6 | head -120
cargo build -p duarouter >/dev/null 2>&1; echo "build rc=$?"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
for c in duarouter_trips; do
  echo "--- $c cand.log (head)"; head -c 1500 "$OUT/$c/cand.log" | tr -d '\r'
  echo "--- $c cand.log (DU02 lines)"; grep -a 'DU02' "$OUT/$c/cand.log" | head -30
done
