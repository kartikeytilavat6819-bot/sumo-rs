#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
C=tools/oracle/cases/tt_bugs_13xxx_ticket13108
$OURS -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/o.ti.xml --fcd-output $OUT/o.fcd.xml --statistic-output $OUT/o.stat.xml --no-step-log true >$OUT/o.log 2>&1
echo "13108 exit=$?"
tail -3 $OUT/o.log
echo "=== tripinfo diff ==="
python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/o.ti.xml 2>&1 | head -5
echo "=== fcd diff ==="
python tools/oracle/compare_sim.py --fcd $C/ref.fcd.xml $OUT/o.fcd.xml 2>&1 | head -5