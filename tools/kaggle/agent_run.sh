#!/bin/bash
# oc118 probe: person stage timing on the parked personTrip access rows (ours vs official).
# Runs on the Linux gate VM from the repo root; writes only under /tmp.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc118
mkdir -p "$OUT"
OUR="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_devices_rerouting_personTrip_access tt_devices_rerouting_personTrip_ride_trip; do
  d="tools/oracle/cases/$c"
  ARGS=$(cat "$d/sim.args")
  echo "===== $c"
  timeout 120 "$OUR" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output "$OUT/$c.ours.ti.xml" --no-step-log true $ARGS >"$OUT/$c.ours.log" 2>&1
  echo "ours rc=$?"
  timeout 120 "$OFF" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output "$OUT/$c.off.ti.xml" --no-step-log true $ARGS >"$OUT/$c.off.log" 2>&1
  echo "official rc=$?"
  echo "--- ref personinfo"; grep -A8 "<personinfo" "$d/ref.tripinfo.xml" | grep -E "personinfo|<stop|<walk|<ride|<access|<transship|<wait" | head -20
  echo "--- ours personinfo"; grep -A8 "<personinfo" "$OUT/$c.ours.ti.xml" | grep -E "personinfo|<stop|<walk|<ride|<access|<transship|<wait" | head -20
  echo "--- official personinfo"; grep -A8 "<personinfo" "$OUT/$c.off.ti.xml" | grep -E "personinfo|<stop|<walk|<ride|<access|<transship|<wait" | head -20
  echo "--- ours log (head)"; head -c 600 "$OUT/$c.ours.log"; echo
  echo "--- official log (head)"; head -c 600 "$OUT/$c.off.log"; echo
done
