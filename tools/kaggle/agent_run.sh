#!/bin/bash
# pt03 measurement (temporary, removed before merge): busride with run_all's exact arguments
set -u
T=/tmp/pt03; rm -rf $T; mkdir -p $T/out
C=tools/oracle/cases/busride
XTRA=$(sed "s#@OUT@#$T/out#g" $C/sim_outputs.txt | tr -d '\r')
$CARGO_TARGET_DIR/debug/sumo -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $T/ti.xml --fcd-output $T/fcd.xml --statistic-output $T/stat.xml --no-step-log true $(cat $C/sim.args) $XTRA >$T/log 2>&1; echo "rc=$?"
python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $T/ti.xml | head -4 | cut -c1-400
grep -o 'id="L1f.0"[^>]*' $T/ti.xml | grep -o 'devices="[^"]*"'
grep -o 'id="L1f.0"[^>]*' $C/ref.tripinfo.xml | grep -o 'devices="[^"]*"'
