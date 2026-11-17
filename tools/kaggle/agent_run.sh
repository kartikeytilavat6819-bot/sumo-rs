#!/bin/bash
# Diagnostics (temporary, removed before merge): emissions edgeData, ours vs pristine C++.
C=tools/oracle/cases/emissions
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p $OUT
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --end 200 --device.emissions.probability 1 --no-step-log true"
for side in ours pristine; do
  mkdir -p $OUT/$side
  printf '%s\n' '<additional>' '    <edgeData id="ed" freq="60" file="edge.xml" type="emissions"/>' '</additional>' > $OUT/$side/edgedata.add.xml
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  $BIN $ARGS --additional-files $OUT/$side/edgedata.add.xml --tripinfo-output $OUT/$side.ti.xml > $OUT/$side.log 2>&1
  echo "== $side exit=$? lines=$(wc -l < $OUT/$side/edge.xml 2>/dev/null)"
  grep -m3 -E 'panicked|Error' $OUT/$side.log
done
echo "== committed ref vs pristine: $(diff $C/ref_edgedata.edge.xml $OUT/pristine/edge.xml | wc -l) diff lines"
echo "== ours vs committed ref:"
diff $C/ref_edgedata.edge.xml $OUT/ours/edge.xml | head -30
