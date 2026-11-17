#!/bin/bash
# mc04 diagnostics (TEMPORARY, removed before the final commit): meso emissions edgedata case, ours vs pristine C++.
C=tools/oracle/cases/emissions
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
OUT=${OUT:-/tmp/mc04}
rm -rf $OUT; mkdir -p $OUT/ours $OUT/pristine
cp -r $C/. $OUT/ours/; cp -r $C/. $OUT/pristine/
ARGS="--end 200 --device.emissions.probability 1 --mesosim true --no-step-log true"
for side in ours pristine; do
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  (cd $OUT/$side && $BIN -n ref_simple.net.xml -r routes.rou --additional-files edgedata.add.xml $ARGS \
     --tripinfo-output ti.xml > sumo.log 2>&1; echo "== $side exit=$?"; grep -m3 -E 'panicked|Error' sumo.log)
done
echo "== devices ours"; grep -o 'devices="[^"]*"' $OUT/ours/ti.xml | head -6
echo "== devices pristine"; grep -o 'devices="[^"]*"' $OUT/pristine/ti.xml | head -6
echo "== devices ref"; grep -o 'devices="[^"]*"' $C/ref_meso_edgedata.tripinfo.xml | head -6
echo "== tripinfo diff ours vs pristine"; diff $OUT/pristine/ti.xml $OUT/ours/ti.xml | head -20
echo "== tripinfo diff pristine vs ref"; diff $C/ref_meso_edgedata.tripinfo.xml $OUT/pristine/ti.xml | head -6
echo "== edge diff ours vs pristine"; diff $OUT/pristine/out_edgedata.xml $OUT/ours/out_edgedata.xml | head -20
echo "== edge diff pristine vs ref"; diff $C/ref_meso_edgedata.edge.xml $OUT/pristine/out_edgedata.xml | head -6
echo "== edge diff count ours"; diff $OUT/pristine/out_edgedata.xml $OUT/ours/out_edgedata.xml | grep -c '^[<>]'
