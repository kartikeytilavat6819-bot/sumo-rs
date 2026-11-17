#!/bin/bash
# pt02 diagnostics (removed before merge): busride ours vs pristine C++, outputs compared against ref_sim_out.
C=tools/oracle/cases/busride
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=${OUT:-/tmp/agent_pt02}
for side in ours pristine; do
  mkdir -p $OUT/$side
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  $BIN -n $C/ref_simple.net.xml -r $C/routes.rou --end 400 --additional-files $C/busride.add.xml \
    --tripinfo-output $OUT/$side/ti.xml --personinfo-output $OUT/$side/personinfo.xml \
    --stop-output $OUT/$side/stop.xml --no-step-log true > $OUT/$side/log.txt 2>&1
  echo "== $side exit=$?  lines=$(wc -l < $OUT/$side/log.txt)"
  grep -m5 -E 'Error|panicked|Warning' $OUT/$side/log.txt | cut -c1-200
done
python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/ours/ti.xml 2>&1 | head -3
python tools/oracle/compare_xml.py $C/ref_sim_out/personinfo.xml $OUT/ours/personinfo.xml 2>&1 | head -3
python tools/oracle/compare_xml.py $C/ref_sim_out/stop.xml $OUT/ours/stop.xml 2>&1 | head -3
