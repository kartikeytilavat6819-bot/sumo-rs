#!/bin/bash
d=tools/oracle/cases/tls_swarm; O=/tmp/z36; mkdir -p $O
RUST_BACKTRACE=1 $CARGO_TARGET_DIR/debug/sumo -n $d/ref_simple.net.xml -r $d/routes.rou --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml --statistic-output $O/stat.xml --no-step-log true $(cat $d/sim.args) > $O/log 2>&1
echo rc=$?; head -40 $O/log
python tools/oracle/compare_sim.py --tripinfo $d/ref.tripinfo.xml $O/ti.xml | head -15
python tools/oracle/compare_stat.py $d/ref.stat.xml $O/stat.xml | head -8
