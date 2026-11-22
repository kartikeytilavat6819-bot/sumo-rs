#!/bin/bash
# oc104 diagnostics: our sumo vs pristine C++ sumo on the fcd-replay cases.
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=/tmp/oc104_fcd
mkdir -p $OUT
for C in tools/oracle/cases/tt_devices_fcd-replay_plain tools/oracle/cases/tt_devices_fcd-replay_minimal tools/oracle/cases/tt_devices_fcd-replay_persons; do
  NAME=$(basename $C)
  EXTRA=$(cat $C/sim.args)
  echo "################ $NAME"
  for side in ours pristine; do
    if [ "$side" = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
    $BIN -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true $EXTRA \
      --tripinfo-output $OUT/$NAME.$side.ti.xml --fcd-output $OUT/$NAME.$side.fcd.xml \
      --statistic-output $OUT/$NAME.$side.stat.xml > $OUT/$NAME.$side.log 2>&1
    echo "== $NAME $side exit=$?"
    tail -3 $OUT/$NAME.$side.log
  done
  echo "== tripinfo diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.ti.xml $OUT/$NAME.ours.ti.xml | head -60
  echo "== fcd diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.fcd.xml $OUT/$NAME.ours.fcd.xml | head -40
  echo "== stat diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.stat.xml $OUT/$NAME.ours.stat.xml | head -40
done
