#!/bin/bash
# lc06 temporary trace: removed before merge
C=tools/oracle/cases
OURS=$CARGO_TARGET_DIR/debug/sumo
CPP=$(bash tools/gcloud/vm_cpp_trace.sh lc06 tools/kaggle/lc06_cpp.patch sumo)
echo "cpp bin: $CPP"
for c in sl2015_lat08; do
  d=$C/$c
  LC06_TRACE=1 $OURS -n $d/ref_simple.net.xml -r $d/routes.rou $(cat $d/sim.args) --no-step-log true 2>&1 | grep -E '^(PO[+-]|LT) ' > /tmp/lc06_ours_$c.txt
  LC06_TRACE=1 $CPP -n $d/ref_simple.net.xml -r $d/routes.rou $(cat $d/sim.args) --no-step-log true 2>&1 | grep -E '^(PO[+-]|LT) ' > /tmp/lc06_cpp_$c.txt
  echo "$c ours=$(wc -l < /tmp/lc06_ours_$c.txt) cpp=$(wc -l < /tmp/lc06_cpp_$c.txt)"
  grep -E '^PO[+-] .*veh=rgt0' /tmp/lc06_cpp_$c.txt > /tmp/lc06_cpp_po.txt
  grep -E '^PO[+-] .*veh=rgt0' /tmp/lc06_ours_$c.txt > /tmp/lc06_ours_po.txt
  echo "rgt0 PO events: ours=$(wc -l < /tmp/lc06_ours_po.txt) cpp=$(wc -l < /tmp/lc06_cpp_po.txt)"
  diff /tmp/lc06_cpp_po.txt /tmp/lc06_ours_po.txt | head -30
  echo "--- first LT diff"
  diff /tmp/lc06_cpp_$c.txt /tmp/lc06_ours_$c.txt | head -8
done
