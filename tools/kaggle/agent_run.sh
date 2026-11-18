#!/bin/bash
# oc63 diagnostic: isolate the netconvert option causing the join mismatch (seed 139).
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
OURS=$CARGO_TARGET_DIR/debug
OFF=$SUMO_PRISTINE_BIN
PY=$(command -v python3 || command -v python)
D=/tmp/oc63diag; rm -rf "$D"; mkdir -p "$D"

NG='--grid --grid.number 3 --grid.length 100 --seed 139 --default.lanenumber 2 --default-junction-type allway_stop --bikelanes.guess true --roundabouts.guess true --roundabouts.visibility-distance 12 --geometry.remove.min-length 5 --tls.guess true --tls.uncontrolled-within true --tls.layout incoming --tls.green.time 30 --tls.yellow.time 3 --tls.allred.time 2 --junctions.join-same 5 --junctions.limit-turn-speed 4 --junctions.internal-link-detail 2 --junctions.left-before-right true --default.right-of-way edgePriority --default.junctions.keep-clear false'
timeout 90 "$OFF/netgenerate" $NG -o "$D/base.net.xml" >"$D/ng.log" 2>&1
echo "netgenerate rc=$? base=$(test -s "$D/base.net.xml" && echo yes || echo no)"

FULL='--geometry.remove true --edges.join true --lefthand true --ramps.guess true --ramps.ramp-length 150 --tls.guess true --tls.guess.joining true --tls.uncontrolled-within true --tls.cycle.time 60 --tls.no-mixed true --tls.default-type actuated --tls.join true --tls.join-dist 20 --junctions.join-dist 20 --default.speed 8.33 --keep-edges.components 1 --remove-edges.isolated true --offset.x 5 --proj.scale 0.9 --default.sidewalk-width 3'

try() { # try <label> <args>
  local label=$1 args=$2
  timeout 90 "$OFF/netconvert" -s "$D/base.net.xml" $args -o "$D/c.net.xml" >"$D/c.log" 2>&1; local cr=$?
  timeout 90 "$OURS/netconvert" -s "$D/base.net.xml" $args -o "$D/r.net.xml" >"$D/r.log" 2>&1; local rr=$?
  if [ $cr -ne $rr ]; then echo "$label EXIT cpp=$cr rust=$rr"; return; fi
  if [ $cr -ne 0 ]; then echo "$label BOTHFAIL rc=$cr"; return; fi
  "$PY" tools/oracle/compare_net.py "$D/c.net.xml" "$D/r.net.xml" > "$D/diff" 2>&1; local dr=$?
  if [ $dr -eq 0 ]; then echo "$label PASS"; else echo "$label FAIL $(head -1 "$D/diff" | cut -c1-120)"; fi
}

echo "=== full logs (progress lines) ==="
timeout 90 "$OFF/netconvert" -s "$D/base.net.xml" $FULL -o "$D/c.net.xml" >"$D/c.log" 2>&1
timeout 90 "$OURS/netconvert" -s "$D/base.net.xml" $FULL -o "$D/r.net.xml" >"$D/r.log" 2>&1
echo "-- cpp --"; grep -iE 'remov|join|nodes' "$D/c.log" | head -20
echo "-- rust --"; grep -iE 'remov|join|nodes' "$D/r.log" | head -20

echo "=== leave-one-out ==="
try FULL "$FULL"
for o in '--geometry.remove true' '--edges.join true' '--lefthand true' '--ramps.guess true' '--tls.guess true' '--tls.join true' '--junctions.join-dist 20' '--offset.x 5' '--proj.scale 0.9' '--keep-edges.components 1' '--remove-edges.isolated true'; do
  a=${FULL/$o/}
  try "minus[$o]" "$a"
done

echo "=== pairs of interest ==="
try 'edges.join+geometry.remove' '--edges.join true --geometry.remove true'
try 'edges.join only' '--edges.join true'
try 'geometry.remove only' '--geometry.remove true'
try 'no opts' ''
