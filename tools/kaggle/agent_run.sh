#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
C=tools/oracle/cases/tt_bugs_13xxx_ticket13108
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OFF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/agent_run; mkdir -p $OUT
echo "=== 13108 ours ==="
RUST_BACKTRACE=1 $OURS -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/o.ti.xml --fcd-output $OUT/o.fcd.xml --statistic-output $OUT/o.stat.xml --no-step-log true >$OUT/o.log 2>&1
echo "exit=$?"
tail -30 $OUT/o.log
echo "=== 13108 official ==="
$OFF -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --tripinfo-output $OUT/p.ti.xml --fcd-output $OUT/p.fcd.xml --statistic-output $OUT/p.stat.xml --no-step-log true >$OUT/p.log 2>&1
echo "exit=$?"
tail -5 $OUT/p.log