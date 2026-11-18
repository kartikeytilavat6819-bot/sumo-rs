#!/bin/bash
# oc63 diagnostic: dump base net structure around node A0 for geometry.remove mismatch.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
OFF=$SUMO_PRISTINE_BIN; OURS=$CARGO_TARGET_DIR/debug
PY=$(command -v python3 || command -v python)
D=/tmp/oc63diag2; rm -rf "$D"; mkdir -p "$D"
NG='--grid --grid.number 3 --grid.length 100 --seed 139 --default.lanenumber 2 --default-junction-type allway_stop --bikelanes.guess true --roundabouts.guess true --roundabouts.visibility-distance 12 --geometry.remove.min-length 5 --tls.guess true --tls.uncontrolled-within true --tls.layout incoming --tls.green.time 30 --tls.yellow.time 3 --tls.allred.time 2 --junctions.join-same 5 --junctions.limit-turn-speed 4 --junctions.internal-link-detail 2 --junctions.left-before-right true --default.right-of-way edgePriority --default.junctions.keep-clear false'
timeout 90 "$OFF/netgenerate" $NG -o "$D/base.net.xml" >"$D/ng.log" 2>&1
echo "=== node A0 ==="
grep -E '<node id="A0"' "$D/base.net.xml"
echo "=== tlLogic A0 ==="
grep -E 'tlLogic id="A0"|tlLogic id="[^"]*A0' "$D/base.net.xml" | head -5
echo "=== edges touching A0 ==="
grep -E '<edge id="[^"]*" from="A0"| to="A0"' "$D/base.net.xml" | head -30
echo "=== crossings near A0 ==="
grep -E '<crossing ' "$D/base.net.xml" | grep -i A0 | head -10
echo "=== cpp with geometry.remove ==="
timeout 90 "$OFF/netconvert" -s "$D/base.net.xml" --geometry.remove true -o "$D/c.net.xml" >"$D/c.log" 2>&1
timeout 90 "$OURS/netconvert" -s "$D/base.net.xml" --geometry.remove true -o "$D/r.net.xml" >"$D/r.log" 2>&1
echo "-- cpp A1A0/A0 edges --"; grep -E '<edge id="A1A0"|<edge id="A0' "$D/c.net.xml" | head -10
echo "-- rust A1A0/A0 edges --"; grep -E '<edge id="A1A0"|<edge id="A0' "$D/r.net.xml" | head -10
echo "-- cpp log tail --"; tail -15 "$D/c.log"
echo "-- rust log tail --"; tail -15 "$D/r.log"
