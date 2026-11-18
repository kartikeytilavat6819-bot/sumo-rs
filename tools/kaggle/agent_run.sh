#!/bin/bash
# oc63 diagnostic: print node removability decisions from our netconvert.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
OFF=$SUMO_PRISTINE_BIN; OURS=$CARGO_TARGET_DIR/debug
D=/tmp/oc63diag4; rm -rf "$D"; mkdir -p "$D"
NG='--grid --grid.number 3 --grid.length 100 --seed 139 --default.lanenumber 2 --default-junction-type allway_stop --roundabouts.guess true --tls.guess true --tls.uncontrolled-within true --tls.layout incoming --junctions.left-before-right true --default.right-of-way edgePriority'
timeout 90 "$OFF/netgenerate" $NG -o "$D/base.net.xml" >"$D/ng.log" 2>&1
echo "ng rc=$? base=$(test -s "$D/base.net.xml" && echo yes || echo no) log:"
head -5 "$D/ng.log"
echo "=== junctions raw (first 4) ==="
grep '<junction ' "$D/base.net.xml" | head -4
echo "=== A0 removability (OC63_DEBUG) ==="
OC63_DEBUG=1 timeout 90 "$OURS/netconvert" -s "$D/base.net.xml" --geometry.remove true -o "$D/r.net.xml" >"$D/r.log" 2>&1
echo "netconvert rc=$?"
grep 'OC63 node' "$D/r.log" | grep -E 'node=A0 |node=A1 |node=B0 ' | head -10
echo "=== all removability reasons (count) ==="
grep 'OC63 node' "$D/r.log" | sed -E 's/.*rem=(true|false) reason=(.*)$/\1 \2/' | sort | uniq -c | head -20
echo "=== log head ==="
head -5 "$D/r.log"
