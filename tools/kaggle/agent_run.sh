#!/bin/bash
c=tools/oracle/cases/elechybrid_overhead; O=/tmp/f03; mkdir -p $O
B=$CARGO_TARGET_DIR/debug/sumo; [ -x $B ] || B=$B.exe
$B -n $c/ref_simple.net.xml -r $c/routes.rou --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml --no-step-log true $(cat $c/sim.args) --elechybrid-output $O/elechybrid.xml --overheadwiresegments-output $O/overheadwires.xml --substations-output $O/substations.xml > $O/log 2>&1
tail -3 $O/log
for s in elechybrid overheadwires substations; do
  echo "== $s"; python3 tools/oracle/compare_xml.py $c/ref.$s.xml $O/$s.xml | head -8
  diff <(sed '1,/-->/d' $c/ref.$s.xml) <(sed '1,/-->/d' $O/$s.xml) | cut -c1-400 | head -16
done
c=tools/oracle/cases/out_tls; O=/tmp/f03t; mkdir -p $O
$B -n $c/ref_simple.net.xml -r $c/routes.rou --no-step-log true $(cat $c/sim.args) --tls-state-output $O/s.xml --tlsswitch-output $O/sw.xml --tlsswitch-states-output $O/ss.xml > $O/log 2>&1
cat $c/tlsout.add.xml | head -20; ls $O
