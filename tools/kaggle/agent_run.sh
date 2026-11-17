#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_delft_campus_crossings
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
$OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
for s in ref ours; do
  if [ $s = ref ]; then F=$D/ref.fcd.xml; else F=$O/ours.fcd.xml; fi
  awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} /id="car8"/{sub(/^ */,""); print t, $0}' $F > $O/$s.car8.txt
done
echo "ref $(wc -l < $O/ref.car8.txt) ours $(wc -l < $O/ours.car8.txt)"
diff $O/ref.car8.txt $O/ours.car8.txt | head -8
echo "--- first 2 ours car8 rows:"; head -2 $O/ours.car8.txt
echo "--- car8 tail ref/ours:"; tail -1 $O/ref.car8.txt; tail -1 $O/ours.car8.txt
