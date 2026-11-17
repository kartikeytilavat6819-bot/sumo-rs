#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_grenoble_tram_bus
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
CPPT=$(bash tools/gcloud/vm_cpp_trace.sh cd01 tools/kaggle/cd01.patch)
SA=$(cat $D/sim.args)
CD01=1 $OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --personinfo-output $O/ours.pi.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
CD01=1 $CPPT -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/cpp.ti.xml --fcd-output $O/cpp.fcd.xml --statistic-output $O/cpp.stat.xml --personinfo-output $O/cpp.pi.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp exit $?"
echo "OURS PWENTRY: $(grep -c 'CD01 PWENTRY' $O/ours.log)  pwL10: $(grep -c 'PWENTRY pwL10' $O/ours.log)"
echo "OURS WALKNEW pwL10:"; grep 'CD01 WALKNEW pwL10' $O/ours.log
echo "OURS PW pwL10:"; grep 'CD01 PW ' $O/ours.log | head -3
echo "OURS CA:"; grep 'CD01 CA' $O/ours.log | head -4
echo "OURS pwL10 log lines:"; grep -c 'pwL10' $O/ours.log
echo "OURS person lines in log (first 5):"; grep 'pwL10' $O/ours.log | head -5
