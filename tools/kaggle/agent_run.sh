#!/bin/bash
# TEMP rr02 measurement (removed before final commit)
D=tools/oracle/cases/city_rotterdam_rail_yard
O=/tmp/rr02; rm -rf $O; mkdir -p $O
CPP=$(bash tools/gcloud/vm_cpp_trace.sh rr02 tools/kaggle/rr02.patch sumo) || { echo "CPP BUILD FAILED"; exit 0; }
echo "cpp: $CPP"
SA=$(cat $D/sim.args)
$CPP -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/cpp.ti.xml --no-step-log true > $O/cpp.log 2>&1; echo "cpp rc=$?"
RR02_DW=1 $CARGO_TARGET_DIR/debug/sumo -n $D/ref_simple.net.xml -r $D/routes.rou $SA --tripinfo-output $O/ours.ti.xml --no-step-log true > $O/ours.log 2>&1; echo "ours rc=$?"
for s in cpp ours; do grep -a '^DW' $O/$s.log | sed -E 's/^(DWR) [0-9.]+ /\1 /' > $O/$s.tr; done
echo "lines cpp=$(wc -l < $O/cpp.tr) ours=$(wc -l < $O/ours.tr)"
echo "=== CPP TRACE"; cut -c1-600 $O/cpp.tr | head -150
echo "=== DIFF"; diff $O/cpp.tr $O/ours.tr | cut -c1-600 | head -80
echo "=== ti diff"; diff <(grep '<tripinfo ' $O/cpp.ti.xml) <(grep '<tripinfo ' $O/ours.ti.xml) | head -10
