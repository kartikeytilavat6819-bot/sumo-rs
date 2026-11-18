#!/bin/bash
# oc26: isolate edge-based trackVehicles bug: one detector, one long interval.
set -u
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
PRIS=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/oc26
mkdir -p $OUT
C=md_vtypes
dir=tools/oracle/cases/$C

# One edge-based trackVehicles detector over the WHOLE sim (single interval),
# so no interval boundaries / resetOnly bookkeeping is involved.
cat > $OUT/single.add.xml <<'EOF'
<additional>
  <edgeData id="single" period="1000" trackVehicles="true" file="out_single.xml"/>
</additional>
EOF

for side in ours pris; do
  rm -f $dir/out_single.xml
  if [ $side = ours ]; then BIN=$OURS; else BIN=$PRIS; fi
  $BIN -n $dir/ref_simple.net.xml -r $dir/routes.rou --end 200 \
       --additional-files $OUT/single.add.xml --no-step-log true > $OUT/$C.$side.log 2>&1
  echo "CASE single side=$side exit=$?"
  if [ -f "$dir/out_single.xml" ]; then cp "$dir/out_single.xml" "$OUT/$C.$side.single.xml"; else echo "  MISSING"; fi
done
if [ -f "$OUT/$C.ours.single.xml" ] && [ -f "$OUT/$C.pris.single.xml" ]; then
  echo "compare: $(python3 tools/oracle/compare_xml.py "$OUT/$C.pris.single.xml" "$OUT/$C.ours.single.xml" 2>&1 | head -1)"
  python3 - "$OUT/$C.pris.single.xml" "$OUT/$C.ours.single.xml" <<'PY'
import sys, xml.etree.ElementTree as ET
for label, path in (("REF", sys.argv[1]), ("CAND", sys.argv[2])):
    root = ET.parse(path).getroot()
    for iv in root.findall("interval"):
        print(f"  {label} {iv.get('begin')}-{iv.get('end')}")
        for e in iv.findall("edge"):
            ss=e.get('sampledSeconds')
            if ss and float(ss)!=0:
                print(f"    {e.get('id')}: ss={ss} entered={e.get('entered')} left={e.get('left')}")
PY
fi
