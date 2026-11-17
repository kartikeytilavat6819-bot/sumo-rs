#!/bin/bash
# pt03 measurement (temporary, removed before merge): busride L1f.0 pedestrian-leader trace, C++ vs ours
set -u
T=/tmp/pt03; rm -rf $T; mkdir -p $T/ref $T/cpp $T/rs
C=tools/oracle/cases/busride
P=$T/prepo; mkdir -p $P; cp -r /home/gate/sumo-cpp/src $P/src
git -C $P init -q; git -C $P add -A; git -C $P -c user.email=pt03@x -c user.name=pt03 commit -qm base >/dev/null
python3 - "$P/src/microsim/MSVehicle.cpp" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
a = '            const double relativePos = lane->getLength() - seen;\n'
assert s.count(a) == 1, ("anchor a", s.count(a))
s = s.replace(a, '            if (getID() == "L1f.0") std::cerr << "PT03 t=" << SIMTIME << " lane=" << lane->getID() << " persons=" << lane->getEdge().getPersons().size() << " hasPed=" << lane->hasPedestrians() << " seen=" << seen << " pos=" << getPositionOnLane() << "\\n";\n' + a)
b = '            if (leader.first != 0) {\n                const double stopSpeed = cfModel.stopSpeed(this, getSpeed(), leader.second - getVehicleType().getMinGap());\n'
assert s.count(b) == 1, ("anchor b", s.count(b))
s = s.replace(b, '            if (getID() == "L1f.0") std::cerr << "PT03 leader_null=" << (leader.first == 0) << " dist=" << leader.second << "\\n";\n' + b)
open(p, 'w').write(s)
PY
git -C $P diff > $T/cpp.patch; echo "patch lines: $(wc -l < $T/cpp.patch)"
CPPBIN=$(bash tools/gcloud/vm_cpp_trace.sh pt03 $T/cpp.patch 2>$T/build.log | tail -1); echo "cpp bin: $CPPBIN"
NET="-n $C/ref_simple.net.xml -r $C/routes.rou $(cat $C/sim.args) --no-step-log true"
OUTS=$(sed "s#@OUT@#$T/ref#g" $C/sim_outputs.txt | tr -d '\r')
$SUMO_PRISTINE_BIN/sumo $NET --fcd-output $T/ref/fcd.xml --tripinfo-output $T/ref/ti.xml $OUTS >$T/ref/log 2>&1; echo "ref rc=$?"
OUTS=$(sed "s#@OUT@#$T/cpp#g" $C/sim_outputs.txt | tr -d '\r')
$CPPBIN $NET --fcd-output $T/cpp/fcd.xml --tripinfo-output $T/cpp/ti.xml $OUTS >$T/cpp/log 2>$T/cpp/trace.txt; echo "cpp rc=$?"
OUTS=$(sed "s#@OUT@#$T/rs#g" $C/sim_outputs.txt | tr -d '\r')
PT03=1 $CARGO_TARGET_DIR/debug/sumo $NET --fcd-output $T/rs/fcd.xml --tripinfo-output $T/rs/ti.xml $OUTS >$T/rs/log 2>$T/rs/trace.txt; echo "rs rc=$?"
for s in ref rs; do
  awk '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART,RLENGTH)} /id="L1f\.0"/{print t, $0}' $T/$s/fcd.xml > $T/$s/bus.txt
done
echo "FCD first diff ref vs rs:"; diff $T/ref/bus.txt $T/rs/bus.txt | head -4 | cut -c1-200
echo "C++ trace lines: $(grep -c PT03 $T/cpp/trace.txt)"; grep PT03 $T/cpp/trace.txt | head -14
echo "RS trace lines: $(grep -c PT03 $T/rs/trace.txt)"; grep PT03 $T/rs/trace.txt | head -14
echo "personinfo rows ref=$(grep -c '<personinfo' $T/ref/personinfo.xml 2>/dev/null) rs=$(grep -c '<personinfo' $T/rs/personinfo.xml 2>/dev/null)"
