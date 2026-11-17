#!/bin/bash
# cd01 measurement (removed before final commit)
D=tools/oracle/cases/city_grenoble_tram_bus
O=/tmp/cd01; mkdir -p $O
OURS=/tmp/target/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
export SUMO_HOME=${SUMO_HOME:-}
SA=$(cat $D/sim.args)
$OURS -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --fcd-output $O/ours.fcd.xml --statistic-output $O/ours.stat.xml --no-step-log true > $O/ours.log 2>&1; echo "ours exit $?"
$CPP -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/cpp.ti.xml --fcd-output $O/cpp.fcd.xml --statistic-output $O/cpp.stat.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp exit $?"
echo "REF:"; grep 'id="L1f.0"' $D/ref.tripinfo.xml | grep -o 'devices="[^"]*"'
echo "OURS:"; grep 'id="L1f.0"' $O/ours.ti.xml | grep -o 'devices="[^"]*"'
echo "CPP:"; grep 'id="L1f.0"' $O/cpp.ti.xml | grep -o 'devices="[^"]*"'
echo "ours tripinfo count: $(grep -c '<tripinfo ' $O/ours.ti.xml) cpp: $(grep -c '<tripinfo ' $O/cpp.ti.xml)"
echo "person-related lines ours:"; grep -c 'person' $O/ours.ti.xml
echo "ours fcd L1f.0 first:"; grep -m1 'vehicle id="L1f.0"' $O/ours.fcd.xml
echo "ours log tail:"; tail -3 $O/ours.log
