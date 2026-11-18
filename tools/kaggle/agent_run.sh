#!/bin/bash
# oc63 diagnostic: exact failing seed 139 net, trace geometry.remove join loop.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
OFF=$SUMO_PRISTINE_BIN; OURS=$CARGO_TARGET_DIR/debug
D=/tmp/oc63diag5; rm -rf "$D"; mkdir -p "$D"
NG='--grid --grid.number 3 --grid.length 100 --seed 139 --default.lanenumber 2 --default-junction-type allway_stop --bikelanes.guess true --roundabouts.guess true --roundabouts.visibility-distance 12 --geometry.remove.min-length 5 --tls.guess true --tls.uncontrolled-within true --tls.layout incoming --tls.green.time 30 --tls.yellow.time 3 --tls.allred.time 2 --junctions.join-same 5 --junctions.limit-turn-speed 4 --junctions.internal-link-detail 2 --junctions.left-before-right true --default.right-of-way edgePriority --default.junctions.keep-clear false'
timeout 90 "$OFF/netgenerate" $NG -o "$D/base.net.xml" >"$D/ng.log" 2>&1
echo "ng rc=$? base=$(test -s "$D/base.net.xml" && echo yes || echo no)"
echo "=== A0 junction ==="
grep '<junction id="A0"' "$D/base.net.xml"
echo "=== rust debug ==="
OC63_DEBUG=1 timeout 90 "$OURS/netconvert" -s "$D/base.net.xml" --geometry.remove true -o "$D/r.net.xml" >"$D/r.log" 2>&1
echo "rc=$?"
grep 'OC63' "$D/r.log" | grep -E 'node=A0 |join at A0|after A1A0|after A0B0|after B0A0' | head -30
echo "=== rust result edges ==="
grep -E '<edge id="A1A0"|<edge id="B0A0"' "$D/r.net.xml"
