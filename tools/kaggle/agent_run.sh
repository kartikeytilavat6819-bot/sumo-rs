#!/bin/bash
# du02 scratch runner: builds duarouter and runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter >/dev/null 2>&1; echo "build rc=$?"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
for f in routes.rou.alt.xml routes.rou.xml; do
  echo "--- trips REF $f"; head -n 30 "$OUT/duarouter_trips/ref/$f" | tr -d '\r'
  echo "--- trips CAND $f"; head -n 30 "$OUT/duarouter_trips/cand/$f" | tr -d '\r'
done
