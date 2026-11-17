#!/bin/bash
# fp01 TEMPORARY: florence pedestrian panic backtrace (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
[ -x "$B.exe" ] && B=$B.exe
cd tools/oracle/cases/city_florence_pedestrians || exit 1
RUST_BACKTRACE=1 $B -n ref_simple.net.xml -r routes.rou --tripinfo-output /tmp/fl.ti.xml --fcd-output /tmp/fl.fcd.xml --statistic-output /tmp/fl.stat.xml --no-step-log true --end 600 > /tmp/fl.log 2>&1
echo "exit=$?"
grep -m3 -E 'panicked at|Error:' /tmp/fl.log
grep -E '^ +[0-9]+: .*(sumo_core|utils)' -A1 /tmp/fl.log | head -90
