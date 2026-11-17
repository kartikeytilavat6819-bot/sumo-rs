#!/bin/bash
# tp01 TEMPORARY: pt2_0 position trace, ours vs ref (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
cd tools/oracle/cases/person_public_car || exit 1
$B -n ref_simple.net.xml -r routes.rou --end 250 --additional-files busline.add.xml --tripinfo-output /tmp/pp_t.xml --fcd-output /tmp/pp_f.xml --no-step-log true > /tmp/pp_log.txt 2>&1
echo "OURS tripinfo:"; grep 'id="pt2_0"' /tmp/pp_t.xml | head -2
echo "REF tripinfo:"; grep 'id="pt2_0"' ref.tripinfo.xml | head -2
echo "OURS fcd tail:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' /tmp/pp_f.xml | tail -6
echo "REF fcd tail:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' ref.fcd.xml | tail -6
echo "OURS log:"; grep -i -E 'warn|error' /tmp/pp_log.txt | head -5
