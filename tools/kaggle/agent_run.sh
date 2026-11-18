#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
C=tools/oracle/cases/tt_bugs_13xxx_ticket13108
RUST_BACKTRACE=1 $OURS -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/o.ti.xml --fcd-output $OUT/o.fcd.xml --statistic-output $OUT/o.stat.xml --no-step-log true >$OUT/o.log 2>&1
rc=$?
echo "MARKER_START rc=$rc"
grep -m1 -n "panicked at" $OUT/o.log
grep -A30 "panicked at" $OUT/o.log | grep "sumo_core::" | head -40
echo "MARKER_END"