#!/bin/bash
# cb01 measurement (remove before final commit)
set -u
C=tools/oracle/cases/city_paris_dense
CB01_TRACE=1 /tmp/target/debug/sumo -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output /tmp/ti.xml --fcd-output /tmp/fcd.xml --statistic-output /tmp/stat.xml --no-step-log true $(cat $C/sim.args) > /tmp/sumo.log 2>&1
echo "sumo exit=$?"
grep TRACE /tmp/sumo.log | head -20
echo "--- ref vs ours veh 3 around t=398"
grep -E 'timestep time="39[78]' -A400 $C/ref.fcd.xml | grep 'vehicle id="3" ' | head -4
grep -E 'timestep time="39[78]' -A400 /tmp/fcd.xml | grep 'vehicle id="3" ' | head -4
