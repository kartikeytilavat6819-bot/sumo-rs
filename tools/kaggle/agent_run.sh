#!/bin/bash
# gr01 measurement (removed before final commit)
D=tools/oracle/cases/ped_persontrip_public
O=/tmp/gr01; mkdir -p $O
OURS=$CARGO_TARGET_DIR/debug/sumo
CPP=$(bash tools/gcloud/vm_cpp_trace.sh gr01 tools/kaggle/gr01.patch sumo | tail -1)
echo "cpp=$CPP"
SA=$(cat $D/sim.args)
for s in cpp ours; do
  if [ $s = cpp ]; then B=$CPP; else B=$OURS; fi
  GR01_TRACE=1 $B -n $D/ref_simple.net.xml -r $D/routes.rou $SA --no-step-log true > $O/$s.log 2>&1; echo "$s exit $?"
done
for s in cpp ours; do echo "===== $s"; grep GR01 $O/$s.log | head -150; done
