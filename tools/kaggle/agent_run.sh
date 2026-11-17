#!/bin/bash
# Diagnostics (temporary, removed before merge): emissions edgeData, ours vs committed pristine reference.
C=tools/oracle/cases/emissions
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p $OUT/ours
printf '%s\n' '<additional>' '    <edgeData id="ed" freq="60" file="edge.xml" type="emissions"/>' '</additional>' > $OUT/ours/edgedata.add.xml
$OURS -n $C/ref_simple.net.xml -r $C/routes.rou --end 200 --device.emissions.probability 1 --no-step-log true --additional-files $OUT/ours/edgedata.add.xml --tripinfo-output $OUT/ours.ti.xml > $OUT/ours.log 2>&1
echo "== ours exit=$?"
grep -m3 -E 'panicked|Error' $OUT/ours.log
echo "== ours edge attrs (first edge):"
grep -m1 -A1 '<edge id' $OUT/ours/edge.xml
echo "== ref edge attrs (first edge):"
grep -m1 -A1 '<edge id' $C/ref_edgedata.edge.xml
echo "== diff lines ours vs ref: $(diff $C/ref_edgedata.edge.xml $OUT/ours/edge.xml | wc -l)"
diff $C/ref_edgedata.edge.xml $OUT/ours/edge.xml | head -30
