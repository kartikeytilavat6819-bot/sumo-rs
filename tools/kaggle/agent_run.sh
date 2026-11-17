#!/bin/bash
# mc04 diagnostics (TEMPORARY, removed before the final commit): run_all's exact command for the meso emissions case.
C=tools/oracle/cases/emissions
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
OUT=${OUT:-/tmp/mc04}
rm -rf $OUT; mkdir -p $OUT/ours $OUT/pristine
cp -r $C/. $OUT/ours/; cp -r $C/. $OUT/pristine/
EXTRA=$(cat $C/sim_meso_edgedata.args | sed "s#tools/oracle/cases/emissions/##g")
for side in ours pristine; do
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  (cd $OUT/$side && $BIN -n ref_simple.net.xml -r routes.rou \
     --tripinfo-output ti.xml --fcd-output fcd.xml --statistic-output stat.xml --no-step-log true $EXTRA \
     > sumo.log 2>&1; echo "== $side exit=$?"; grep -m3 -E 'panicked|Error' sumo.log)
done
echo "== devices ours"; grep -o 'devices="[^"]*"' $OUT/ours/ti.xml | head -5
echo "== devices pristine"; grep -o 'devices="[^"]*"' $OUT/pristine/ti.xml | head -5
echo "== devices ref"; grep -o 'devices="[^"]*"' $C/ref_meso_edgedata.tripinfo.xml | head -5
echo "== compare_sim ours vs ref"; python tools/oracle/compare_sim.py --tripinfo $C/ref_meso_edgedata.tripinfo.xml $OUT/ours/ti.xml 2>&1 | head -12
echo "== compare_sim pristine vs ref"; python tools/oracle/compare_sim.py --tripinfo $C/ref_meso_edgedata.tripinfo.xml $OUT/pristine/ti.xml 2>&1 | head -12
echo "== edge ours vs ref"; python tools/oracle/compare_xml.py $C/ref_meso_edgedata.edge.xml $OUT/ours/out_edgedata.xml 2>&1 | head -6
echo "== edge pristine vs ref"; python tools/oracle/compare_xml.py $C/ref_meso_edgedata.edge.xml $OUT/pristine/out_edgedata.xml 2>&1 | head -6
echo "== ours tripinfo v3 line"; grep 'id="v3"' $OUT/ours/ti.xml | head -2
echo "== pristine tripinfo v3 line"; grep 'id="v3"' $OUT/pristine/ti.xml | head -2
