#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_grenoble_tram_bus
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
CPPT=$(bash tools/gcloud/vm_cpp_trace.sh cd01 tools/kaggle/cd01.patch)
SA=$(cat $D/sim.args)
CD01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
CD01=1 $CPPT -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/cpp.ti.xml --fcd-output $O/cpp.fcd.xml --statistic-output $O/cpp.stat.xml --personinfo-output $O/cpp.pi.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp exit $?"
echo "OURS AW:"; grep 'CD01 AW' $O/ours.log
echo "CPP AW:"; grep 'CD01 AW' $O/cpp.log
echo "OURS walk/ride pwL10:"; grep -A3 'personinfo id="pwL10"' $O/ours.pi.xml | head -6
echo "CPP walk/ride pwL10:"; grep -A3 'personinfo id="pwL10"' $O/cpp.pi.xml | head -6
