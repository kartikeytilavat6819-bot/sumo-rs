#!/bin/bash
# pt03 measurement (temporary, removed before merge): busride boarding devices
set -u
T=/tmp/pt03; rm -rf $T; mkdir -p $T/rs
C=tools/oracle/cases/busride
OUTS=$(sed "s#@OUT@#$T/rs#g" $C/sim_outputs.txt | tr -d '\r')
PT03=1 $CARGO_TARGET_DIR/debug/sumo -n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --no-step-log true --tripinfo-output $T/rs/ti.xml $OUTS >$T/rs/log 2>$T/rs/trace.txt; echo "rs rc=$?"
grep PT03 $T/rs/trace.txt | head -20 | cut -c1-200
echo "PT03 lines: $(grep -c PT03 $T/rs/trace.txt)"
grep 'L1f.0' $T/rs/ti.xml | grep -o 'devices="[^"]*"'
