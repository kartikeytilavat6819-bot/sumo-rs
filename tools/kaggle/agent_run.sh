#!/bin/bash
# oc104 diagnostics: our sumo vs pristine C++ sumo on the plain fcd-replay case.
C=tools/oracle/cases/tt_devices_fcd-replay_plain
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=/tmp/oc104_fcd
mkdir -p $OUT
EXTRA=$(cat $C/sim.args)
for side in ours pristine; do
  if [ "$side" = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  $BIN -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true $EXTRA \
    --tripinfo-output $OUT/$side.ti.xml --fcd-output $OUT/$side.fcd.xml \
    --statistic-output $OUT/$side.stat.xml > $OUT/$side.log 2>&1
  echo "== $side exit=$?"
  tail -3 $OUT/$side.log
done
echo "== tripinfo diff (pristine -> ours)"
diff -u $OUT/pristine.ti.xml $OUT/ours.ti.xml | head -60
echo "== fcd diff (pristine -> ours)"
diff -u $OUT/pristine.fcd.xml $OUT/ours.fcd.xml | head -40
echo "== stat diff (pristine -> ours)"
diff -u $OUT/pristine.stat.xml $OUT/ours.stat.xml | head -40
