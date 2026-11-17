#!/bin/bash
# tp01 TEMPORARY: pt2_0 divergence start (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
cd tools/oracle/cases/person_public_car || exit 1
$B -n ref_simple.net.xml -r routes.rou --end 250 --additional-files busline.add.xml --tripinfo-output /tmp/pp_t.xml --fcd-output /tmp/pp_f.xml --no-step-log true > /tmp/pp_log.txt 2>&1
echo "OURS first:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' /tmp/pp_f.xml | head -12 | cut -c1-200
echo "REF first:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' ref.fcd.xml | head -12 | cut -c1-200
echo "OURS 100-110:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' /tmp/pp_f.xml | sed -n '45,52p' | cut -c1-200
echo "REF 100-110:"; awk '/<timestep/{t=$0} /id="pt2_0"/{print t" | "$0}' ref.fcd.xml | sed -n '45,52p' | cut -c1-200
