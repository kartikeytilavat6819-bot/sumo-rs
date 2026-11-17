#!/bin/bash
# TEMPORARY trace for peds_crossing speedFactor (removed before merge)
C=tools/oracle/cases/peds_crossing
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
$OURS -n $C/ref_simple.net.xml -r $C/routes.rou --end 10 --no-step-log true --tripinfo-output /tmp/sf.ti.xml > /tmp/sf.log 2>&1
echo "exit=$?"
grep SFDBG /tmp/sf.log | head -30
