#!/bin/bash
# TEMPORARY peds_crossing fcd timeline (removed before merge)
C=tools/oracle/cases/peds_crossing
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --end 400 --no-step-log true"
$OURS $ARGS --tripinfo-output /tmp/pc_ours.ti.xml --fcd-output /tmp/pc_ours.fcd.xml > /tmp/pc_ours.log 2>&1
$SUMO_PRISTINE_BIN/sumo $ARGS --tripinfo-output /tmp/pc_ref.ti.xml --fcd-output /tmp/pc_ref.fcd.xml > /tmp/pc_ref.log 2>&1
for s in ours ref; do
  echo "=== $s first 3 timesteps with persons"
  grep -n -E "<timestep|<person " /tmp/pc_$s.fcd.xml | head -16
done
echo "=== ours fcd lines 20-40"; sed -n 20,40p /tmp/pc_ours.fcd.xml
echo "=== ref fcd lines 20-40"; sed -n 20,40p /tmp/pc_ref.fcd.xml
