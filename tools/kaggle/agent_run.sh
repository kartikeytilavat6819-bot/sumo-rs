#!/bin/bash
# Gate diagnostic for oc55: full panic output for the two jumping-stop cases.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo"
[ -x "$SUMO" ] || SUMO="$BIN/sumo.exe"
OUT="${TMPDIR:-/tmp}/oc55-spot"
rm -rf "$OUT"; mkdir -p "$OUT" || exit 1
for c in tt_extended_stops_jumps_jump2 tt_extended_stops_jumps_trip_jump2; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  echo "===== $c ====="
  echo "flags: $flags"
  echo "route:"
  cat "$d/routes.rou"
  RUST_BACKTRACE=1 timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --no-step-log true >"$OUT/$c.log" 2>&1
  echo "--- log ---"
  head -60 "$OUT/$c.log"
done
