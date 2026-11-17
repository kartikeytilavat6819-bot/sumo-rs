#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_delft_campus_crossings
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
$OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
for s in ref ours; do
  if [ $s = ref ]; then F=$D/ref.fcd.xml; else F=$O/ours.fcd.xml; fi
  tr -d '\r' < $F | awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} /<(vehicle|person) /{ if (t+0==365 || t+0==366) { line=$0; sub(/^ */,"",line); x=line; match(x,/ x="[^"]*"/); xv=substr(x,RSTART+4,RLENGTH-5)+0; match(x,/ y="[^"]*"/); yv=substr(x,RSTART+4,RLENGTH-5)+0; if (xv>905 && xv<945 && yv>735 && yv<770) print t, line } }' > $O/$s.near.txt
done
echo "== ref near (t 365/366)"; cat $O/ref.near.txt
echo "== ours near (t 365/366)"; cat $O/ours.near.txt
