#!/bin/bash
# fp01 TEMPORARY: florence car15 timeLoss measurement (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
[ -x "$B.exe" ] && B=$B.exe
cd tools/oracle/cases/city_florence_pedestrians || exit 1
$B -n ref_simple.net.xml -r routes.rou --tripinfo-output /tmp/fl.ti.xml --fcd-output /tmp/fl.fcd.xml --statistic-output /tmp/fl.stat.xml --no-step-log true --end 600 > /tmp/fl.log 2>&1
echo "exit=$?"
diff ref.fcd.xml /tmp/fl.fcd.xml | head -12
echo "--- tripinfo car15 ref vs cand"
grep 'id="car15"' ref.tripinfo.xml
grep 'id="car15"' /tmp/fl.ti.xml
echo "--- car15 fcd first 6 lines ref / cand"
grep -c 'id="car15"' ref.fcd.xml /tmp/fl.fcd.xml
grep 'id="car15"' ref.fcd.xml | head -n 4
grep 'id="car15"' /tmp/fl.fcd.xml | head -n 4
