#!/bin/bash
# pt03 measurement (temporary, removed before merge): busride L1f.0 per-step speed, C++ vs ours
set -u
T=/tmp/pt03; rm -rf $T; mkdir -p $T/cpp $T/rs
C=tools/oracle/cases/busride
NET="-n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --no-step-log true --fcd-output"
OUTS=$(sed "s#@OUT@#$T/cpp#g" $C/sim_outputs.txt | tr -d '\r')
$SUMO_PRISTINE_BIN/sumo $NET $T/cpp/fcd.xml --tripinfo-output $T/cpp/ti.xml $OUTS >$T/cpp/log 2>&1; echo "cpp rc=$?"
OUTS=$(sed "s#@OUT@#$T/rs#g" $C/sim_outputs.txt | tr -d '\r')
$CARGO_TARGET_DIR/debug/sumo $NET $T/rs/fcd.xml --tripinfo-output $T/rs/ti.xml $OUTS >$T/rs/log 2>&1; echo "rs rc=$?"
for s in cpp rs; do
  awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART,RLENGTH)} /id="L1f\.0"/{print t, $0}' $T/$s/fcd.xml > $T/$s/bus.txt
done
echo "bus rows cpp=$(wc -l < $T/cpp/bus.txt) rs=$(wc -l < $T/rs/bus.txt)"
echo "FIRST DIFF (cpp < > rs):"
diff $T/cpp/bus.txt $T/rs/bus.txt | head -8 | cut -c1-240
echo "personinfo rows cpp=$(grep -c '<personinfo' $T/cpp/personinfo.xml 2>/dev/null) rs=$(grep -c '<personinfo' $T/rs/personinfo.xml 2>/dev/null)"
