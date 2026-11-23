#!/bin/bash
# oc118 probe: route-probe expected-error rows, ours vs official (exit code and stderr head).
# Runs on the Linux gate VM from the repo root; writes only under /tmp.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc118
mkdir -p "$OUT"
OUR="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_output_route_probe_devnull_output tt_output_route_probe_null_output tt_output_route_probe_odd_interval \
         tt_output_route_probe_two_same tt_output_route_probe_two_same_b5 tt_output_route_probe_two_same_begin \
         tt_output_route_probe_vTypes; do
  d="tools/oracle/cases/$c"
  ARGS=$(cat "$d/sim.args")
  echo "===== $c"
  (cd "$d" && timeout 120 "$OUR" -n ref_simple.net.xml -r routes.rou $ARGS >"$OUT/$c.ours.out" 2>"$OUT/$c.ours.err"; echo "ours rc=$?")
  (cd "$d" && timeout 120 "$OFF" -n ref_simple.net.xml -r routes.rou $ARGS >"$OUT/$c.off.out" 2>"$OUT/$c.off.err"; echo "official rc=$?")
  echo "--- ours stderr"; head -c 300 "$OUT/$c.ours.err"; echo
  echo "--- official stderr"; head -c 300 "$OUT/$c.off.err"; echo
done
