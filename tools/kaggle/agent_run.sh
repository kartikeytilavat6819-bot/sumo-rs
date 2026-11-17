#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_delft_campus_crossings
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
CD01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
for s in ref ours; do
  if [ $s = ref ]; then F=$D/ref.fcd.xml; else F=$O/ours.fcd.xml; fi
  tr -d '\r' < $F | awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)+0} /<(vehicle|person) /{ line=$0; sub(/^ */,"",line); match(line,/id="[^"]*"/); id=substr(line,RSTART+4,RLENGTH-5); print t, id, line }' | sort -k1,1n -k2,2 > $O/$s.all.txt
done
echo "ref rows $(wc -l < $O/ref.all.txt) ours rows $(wc -l < $O/ours.all.txt)"
diff $O/ref.all.txt $O/ours.all.txt > $O/all.diff
echo "diff lines $(wc -l < $O/all.diff)"
echo "first diff entries (ref):"; grep '^<' $O/all.diff | head -12 | cut -c1-200
echo "first diff entries (ours):"; grep '^>' $O/all.diff | head -12 | cut -c1-200
echo "diff rows per time (first 10 times):"; grep -E '^[<>]' $O/all.diff | awk '{print $2}' | sort -n | uniq -c | head -10
