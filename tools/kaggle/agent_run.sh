#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
C=tools/oracle/cases/tt_bugs_13xxx_ticket13108
RUST_BACKTRACE=full $OURS -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/o.ti.xml --fcd-output $OUT/o.fcd.xml --statistic-output $OUT/o.stat.xml --no-step-log true >$OUT/o.log 2>&1
echo "exit=$?"
grep -n "sumo_core::" $OUT/o.log | grep -i "router\|routing\|intermodal\|pedestrian\|transpor" | head -30