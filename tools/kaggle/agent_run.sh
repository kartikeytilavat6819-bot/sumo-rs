#!/bin/bash
# gr01 measurement (removed before final commit)
D=tools/oracle/cases/ped_persontrip_public
O=/tmp/gr01; mkdir -p $O
OURS=$CARGO_TARGET_DIR/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
for v in sim sim_carwalk; do
  SA=$(cat $D/$v.args)
  for s in cpp ours; do
    if [ $s = cpp ]; then B=$CPP; else B=$OURS; fi
    $B -n $D/ref_simple.net.xml -r $D/routes.rou $SA --fcd-output $O/$v.$s.fcd.xml --personinfo-output $O/$v.$s.pi.xml --vehroute-output $O/$v.$s.vr.xml --no-step-log true > $O/$v.$s.log 2>&1; echo "$v $s exit $?"
  done
  echo "=== $v personinfo diff"; diff <(grep -v -e '^<!--' -e generated $O/$v.cpp.pi.xml | sed -n '/<tripinfos/,$p') <(sed -n '/<tripinfos/,$p' $O/$v.ours.pi.xml) | head -30
  echo "=== $v vehroute diff"; diff <(sed -n '/<routes/,$p' $O/$v.cpp.vr.xml) <(sed -n '/<routes/,$p' $O/$v.ours.vr.xml) | head -40
  echo "=== $v fcd diff"; diff <(sed -n '/<fcd-export/,$p' $O/$v.cpp.fcd.xml) <(sed -n '/<fcd-export/,$p' $O/$v.ours.fcd.xml) | head -30
  echo "=== $v ours log"; head -20 $O/$v.ours.log
done
echo "=== cpp vehroute sim"; sed -n '/<routes/,$p' $O/sim.cpp.vr.xml | grep -v '<vehicle\|<route \|</vehicle\|<stop' | head -60
