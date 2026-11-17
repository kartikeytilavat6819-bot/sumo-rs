#!/usr/bin/env bash
# TEMPORARY (tx03): taxi/ref_closest panic backtrace. Remove before merge.
set -u
D=tools/oracle/cases/taxi
RUST_BACKTRACE=1 $CARGO_TARGET_DIR/debug/sumo -n $D/ref_simple.net.xml -r $D/routes.rou $(cat $D/sim_closest.args) \
    --tripinfo-output /tmp/tx03.ti.xml --no-step-log true > /tmp/tx03.log 2>&1
echo "exit=$?"
grep -n -A45 "panicked" /tmp/tx03.log | cut -c1-220 | head -90
