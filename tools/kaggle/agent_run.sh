#!/bin/bash
# TEMPORARY peds_crossing comparison (removed before merge)
C=tools/oracle/cases/peds_crossing
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --end 400 --no-step-log true"
$OURS $ARGS --tripinfo-output /tmp/pc_ours.ti.xml --fcd-output /tmp/pc_ours.fcd.xml > /tmp/pc_ours.log 2>&1
echo "ours exit=$?"
$SUMO_PRISTINE_BIN/sumo $ARGS --tripinfo-output /tmp/pc_ref.ti.xml --fcd-output /tmp/pc_ref.fcd.xml > /tmp/pc_ref.log 2>&1
echo "pristine exit=$?"
echo "--- ours personinfo"
grep -E "personinfo|<walk" /tmp/pc_ours.ti.xml | head -24
echo "--- pristine personinfo"
grep -E "personinfo|<walk" /tmp/pc_ref.ti.xml | head -24
