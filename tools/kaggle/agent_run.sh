#!/bin/bash
# oc46 option-coverage audit measurement
O=/tmp/oc46out; rm -rf $O; mkdir -p $O
D=tools/oracle/cases/real
OURS=$CARGO_TARGET_DIR/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
for side in cpp ours; do
  if [ $side = cpp ]; then B=$CPP; else B=$OURS; fi
  echo "=== $side ==="
  $B -n $D/ref_simple.net.xml -r $D/routes.rou --no-step-log true --end 20 \
     --movereminder-output $O/$side.mr.xml --movereminder-output.vehicles v0 \
     --tripinfo-output $O/$side.ti.xml > $O/$side.log 2>&1
  echo "rc=$?"
  head -6 $O/$side.log
  echo "mr: $(ls -l $O/$side.mr.xml 2>&1 | head -1)"
  [ -f $O/$side.mr.xml ] && head -4 $O/$side.mr.xml
done
echo "=== tripinfo diff ==="
diff $O/cpp.ti.xml $O/ours.ti.xml 2>&1 | head -10
