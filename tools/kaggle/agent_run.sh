#!/bin/bash
# oc40 netimport shortcut audit diagnostics.
set -u
OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug/netconvert
REF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/netconvert
ROOT=$(pwd)
OUT=/tmp/oc40_out
rm -rf "$OUT"; mkdir -p "$OUT"
cd "$OUT"

# minimal net with one edge/lane
cat > nodes.nod.xml <<'EOF'
<nodes><node id="n0" x="0" y="0"/><node id="n1" x="100" y="0"/></nodes>
EOF
cat > edges.edg.xml <<'EOF'
<edges><edge id="e0" from="n0" to="n1" numLanes="1" speed="13.9"/></edges>
EOF
cat > poi.add.xml <<'EOF'
<additional>
  <poi id="pOnLane" lane="e0_0" pos="50"/>
  <poi id="pOnLaneLat" lane="e0_0" pos="-5" posLat="1.5"/>
</additional>
EOF

echo "=== POI-on-lane additional file ==="
for side in ref ours; do
  if [ "$side" = ref ]; then BIN=$REF; else BIN=$OURS; fi
  $BIN -n nodes.nod.xml -e edges.edg.xml --polygon-files poi.add.xml -o $side.net.xml > $side.log 2>&1
  echo "$side exit=$?"
  grep -iE 'poi|lane|panic|error|warn' $side.log | head -8
done
if [ -f ref.net.xml ] && [ -f ours.net.xml ]; then
  grep -E '<poi |lane=' ref.net.xml | head -8 > ref.poi; grep -E '<poi |lane=' ours.net.xml | head -8 > ours.poi
  diff ref.poi ours.poi | head -20
else
  echo "MISSING net: ref=$(test -f ref.net.xml && echo y || echo n) ours=$(test -f ours.net.xml && echo y || echo n)"
fi

echo "=== import cases oracle ==="
bash "$ROOT/tools/oracle/run_importers.sh" "$OURS" "$REF" "$OUT/imports" 2>&1 | tail -40
