#!/bin/bash
# Gate helper for oc68: backtraces + first diffs for the railway_repair failures.
set -u
ROOT=$(cd "$(dirname "$0")/../.." && pwd)
cd "$ROOT" || exit 1
export RUST_BACKTRACE=1
export NETOPTS2_OUT=/tmp/oc68_rail
NAMES=""
for d in tools/oracle/cases/itt_nc_netconvert_function_railway_repair_* tools/oracle/cases/itt_nc_netconvert_function_railway_signals_guess_by_stops_split; do
  NAMES="$NAMES $(basename "$d")"
done
bash tools/oracle/run_netopts_new.sh $NAMES >/dev/null 2>&1
echo "=== VERDICTS ==="
grep -E 'railway_' "$NETOPTS2_OUT/summary.tsv"
echo "=== PANIC BACKTRACES ==="
for row in $(grep -E 'railway_.*PANIC' "$NETOPTS2_OUT/summary.tsv" | cut -f1 | head -4); do
  echo "--- $row"
  grep -m1 -A14 panicked "$NETOPTS2_OUT/$row/rust.log"
done
echo "=== FAIL DIFFS ==="
for row in $(grep -E 'railway_.*FAIL' "$NETOPTS2_OUT/summary.tsv" | cut -f1 | head -6); do
  echo "--- $row"
  head -8 "$NETOPTS2_OUT/$row/net.log" 2>/dev/null
done
echo "=== DONE ==="
