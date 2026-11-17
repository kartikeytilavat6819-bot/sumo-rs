#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_delft_campus_crossings
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
$OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
grep 'id="car8"' $D/ref.fcd.xml | sed 's/^ *//' > $O/ref.car8.txt
grep 'id="car8"' $O/ours.fcd.xml | sed 's/^ *//' > $O/ours.car8.txt
echo "ref lines $(wc -l < $O/ref.car8.txt) ours lines $(wc -l < $O/ours.car8.txt)"
diff $O/ref.car8.txt $O/ours.car8.txt | head -12
echo "--- ours tripinfo car8:"; grep 'id="car8"' $O/ours.ti.xml | head -2
echo "--- ref personinfo count ours/ref:"; grep -c '<personinfo' $O/ours.pi.xml $D/ref.personinfo.xml
