#!/bin/bash
# dc01 temp measurement script (removed before the final commit)
D=tools/oracle/cases/city_delft_campus_crossings
O=/tmp/dc01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
CPPT=$(bash tools/gcloud/vm_cpp_trace.sh dc01 tools/kaggle/dc01.patch)
echo "cpp bin: $CPPT"
DC01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --fcd-output $O/ours.fcd.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
DC01=1 $CPPT -n $D/ref_simple.net.xml -r $D/routes.rou $SA --fcd-output $O/cpp.fcd.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp exit $?"
for s in cpp ours; do
  tr -d '\r' < $O/$s.fcd.xml | awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)+0} /<(vehicle|person) /{ line=$0; sub(/^ */,"",line); match(line,/id="[^"]*"/); id=substr(line,RSTART+4,RLENGTH-5); print t, id, line }' | sort -s -k1,1n -k2,2 > $O/$s.all.txt
done
diff $O/cpp.all.txt $O/ours.all.txt > $O/all.diff
echo "diff lines $(wc -l < $O/all.diff)"
echo "first diffs:"; grep -E '^[<>]' $O/all.diff | head -16 | cut -c1-230
for s in cpp ours; do echo "== $s p173/car43"; awk '$1>=528 && $1<=531 && ($2=="person173" || $2=="car43")' $O/$s.all.txt | cut -c1-200; done
echo "== CPP trace"; grep -a '^DC01' $O/cpp.log | head -80
echo "== OURS trace"; grep -a '^DC01' $O/ours.log | head -80
