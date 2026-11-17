#!/bin/bash
# TEMPORARY peds_crossing diff (removed before merge)
C=tools/oracle/cases/peds_crossing
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --end 400 --no-step-log true"
$OURS $ARGS --tripinfo-output /tmp/pc_ours.ti.xml --fcd-output /tmp/pc_ours.fcd.xml --statistic-output /tmp/pc_ours.stat.xml > /tmp/pc_ours.log 2>&1
echo "ours exit=$?"
$SUMO_PRISTINE_BIN/sumo $ARGS --tripinfo-output /tmp/pc_ref.ti.xml --fcd-output /tmp/pc_ref.fcd.xml --statistic-output /tmp/pc_ref.stat.xml > /tmp/pc_ref.log 2>&1
echo "pristine exit=$?"
for f in fcd ti; do
  echo "=== $f: $(diff /tmp/pc_ours.$f.xml /tmp/pc_ref.$f.xml | grep -c '^[<>]') changed lines"
  diff /tmp/pc_ours.$f.xml /tmp/pc_ref.$f.xml | grep -v -E "generated on|output value" | head -30
done
