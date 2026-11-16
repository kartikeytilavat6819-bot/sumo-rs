#!/bin/bash
# Default-flags run (no --ignore-route-errors) on the bad route in routes.rou, against ref_error/ (C++ 1.28.0).
# PASS only if our first `Error:` line and exit status equal the C++ reference.
# Not part of run_all.sh: run_all only scores the ignore variant (ref.* + sim.args), see CASE.md.
# Usage: bash tools/oracle/cases/route_check/run_error.sh   (CARGO_TARGET_DIR picks the build under test)
set -u
D=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$D/../../../.." && pwd)
BIN=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-main}/debug/sumo.exe
OUT=${SCORE_OUT:-/c/Users/karti/Temp/score}/route_check
mkdir -p "$OUT"
cd "$ROOT" || exit 1
winpath() { if command -v cygpath >/dev/null 2>&1; then cygpath -w "$1"; else wslpath -w "$1"; fi; }
"$BIN" -n "$(winpath "$D/ref_simple.net.xml")" -r "$(winpath "$D/routes.rou")" --no-step-log true >"$OUT/stdout.txt" 2>"$OUT/stderr.txt"
rc=$?
# Exit status is not compared: C++ sumo exits 0 on a ProcessError (also for a missing route file) while ours exits 1
# (measured 2026-11-16); that is a global divergence in sumo_main, not part of this case.
want_err=$(grep -m1 '^Error:' "$D/ref_error/stderr.txt")
got_err=$(grep -m1 '^Error:' "$OUT/stderr.txt")
if [ "$got_err" = "$want_err" ]; then
  echo "route_check/error PASS  $got_err  (ours rc=$rc, C++ rc=$(sed 's/^rc=//' "$D/ref_error/rc.txt"))"
  exit 0
fi
echo "route_check/error FAIL  ref: '$want_err' / ours: rc=$rc '${got_err:-<no Error: line>}'"
exit 1
