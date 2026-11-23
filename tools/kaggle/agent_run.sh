#!/bin/bash
# oc118 probe: device list of bus0 in the persons rows (ours vs official, same flags incl. fcd).
# Runs on the Linux gate VM from the repo root; writes only under /tmp.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc118
mkdir -p "$OUT"
OUR="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in persons persons_router_pending; do
  d="tools/oracle/cases/$c"
  ARGS=$(cat "$d/sim.args")
  echo "===== $c"
  timeout 300 "$OUR" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$OUT/$c.ours.ti.xml" \
    --fcd-output "$OUT/$c.ours.fcd.xml" --no-step-log true $ARGS >"$OUT/$c.ours.log" 2>&1
  echo "ours rc=$?"
  timeout 300 "$OFF" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$OUT/$c.off.ti.xml" \
    --fcd-output "$OUT/$c.off.fcd.xml" --no-step-log true $ARGS >"$OUT/$c.off.log" 2>&1
  echo "official rc=$?"
  echo "--- ref bus0"; grep -E 'tripinfo id="bus0"' "$d/ref.tripinfo.xml" | grep -o 'devices="[^"]*"'
  echo "--- ours bus0"; grep -E 'tripinfo id="bus0"' "$OUT/$c.ours.ti.xml" | grep -o 'devices="[^"]*"'
  echo "--- official bus0"; grep -E 'tripinfo id="bus0"' "$OUT/$c.off.ti.xml" | grep -o 'devices="[^"]*"'
  echo "--- ours log"; head -c 400 "$OUT/$c.ours.log"; echo
done
