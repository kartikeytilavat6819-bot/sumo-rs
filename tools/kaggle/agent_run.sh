#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
C=tools/oracle/cases/tt_bugs_16xxx_ticket16641
$OURS -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/o.ti.xml --fcd-output $OUT/o.fcd.xml --statistic-output $OUT/o.stat.xml --no-step-log true >$OUT/o.log 2>&1
echo "exit=$?"
echo "=== debug lines ==="
grep -i "DEBUG" $OUT/o.log | head
echo "=== tripinfo diff summary ==="
python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/o.ti.xml 2>&1 | head -1