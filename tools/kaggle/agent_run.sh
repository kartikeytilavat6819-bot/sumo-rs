#!/bin/bash
# du02 scratch runner: builds duarouter and runs the duarouter route oracle. Removed before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
cargo build -p duarouter >/dev/null 2>&1; echo "build rc=$?"
bash tools/oracle/run_duarouter.sh
OUT=${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}
BIN=${BIN:-${CARGO_TARGET_DIR:-/tmp/target}/debug/duarouter}
if command -v valgrind >/dev/null 2>&1; then
  echo "--- valgrind trips"
  valgrind --error-limit=no -q "$BIN" -n tools/oracle/cases/trips/ref_simple.net.xml --trip-files tools/oracle/cases/duarouter_trips/trips.xml --output-file "$OUT/vg_routes.rou.xml" --no-step-log true 2>&1 | grep -v '^==[0-9]*== *$' | head -80
elif command -v gdb >/dev/null 2>&1; then
  echo "--- gdb trips"
  gdb -batch -ex run -ex bt --args "$BIN" -n tools/oracle/cases/trips/ref_simple.net.xml --trip-files tools/oracle/cases/duarouter_trips/trips.xml --output-file "$OUT/vg_routes.rou.xml" --no-step-log true 2>&1 | grep -E '^#' | head -60
else
  echo "--- no valgrind/gdb"
fi
