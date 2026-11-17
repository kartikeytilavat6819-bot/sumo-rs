#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_grenoble_tram_bus
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
CPPT=$(bash tools/gcloud/vm_cpp_trace.sh cd01 tools/kaggle/cd01.patch)
SA=$(cat $D/sim.args)
CD01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
CD01=1 $CPPT -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/cpp.ti.xml --fcd-output $O/cpp.fcd.xml --statistic-output $O/cpp.stat.xml --personinfo-output $O/cpp.pi.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp exit $?"
echo "OURS LAW:"; grep 'CD01 LAW' $O/ours.log | head -8
echo "OURS CAND:"; grep 'CD01 CAND' $O/ours.log | head -12
echo "CPP LAW:"; grep 'CD01 LAW' $O/cpp.log | head -8
echo "CPP CAND:"; grep 'CD01 CAND' $O/cpp.log | head -12
echo "OURS ADD: $(grep -c 'CD01 ADD' $O/ours.log)  CPP ADD: $(grep -c 'CD01 ADD' $O/cpp.log)"
