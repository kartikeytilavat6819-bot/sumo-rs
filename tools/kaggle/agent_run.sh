#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_grenoble_tram_bus
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
SA=$(cat $D/sim.args)
CD01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
echo "RR lines: $(grep -c 'CD01 RR' $O/ours.log)"
grep 'CD01 RR' $O/ours.log | head -2
echo "FRAMES:"; grep -A80 'CD01 RR' $O/ours.log | grep -E 'sumo_core|at \./' | head -60
