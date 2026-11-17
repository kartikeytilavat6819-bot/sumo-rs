#!/bin/bash
# pt03 measurement (temporary, removed before merge): busride L1f.0 pedestrian-leader trace, C++ vs ours
set -u
T=/tmp/pt03; rm -rf $T; mkdir -p $T/ref $T/cpp $T/rs
C=tools/oracle/cases/busride
P=$T/prepo; mkdir -p $P; cp -r /home/gate/sumo-cpp/src $P/src
git -C $P init -q; git -C $P add -A; git -C $P -c user.email=pt03@x -c user.name=pt03 commit -qm base >/dev/null
python3 - "$P/src/microsim/MSVehicle.cpp" "$P/src/microsim/transportables/MSPModel_Interacting.cpp" <<'PY'
import sys
p = sys.argv[1]
s = open(p).read()
a = '            const double relativePos = lane->getLength() - seen;\n'
assert s.count(a) == 1, ("anchor a", s.count(a))
s = s.replace(a, '            if (getID() == "L1f.0") std::cerr << "PT03 t=" << SIMTIME << " lane=" << lane->getID() << " persons=" << lane->getEdge().getPersons().size() << " hasPed=" << lane->hasPedestrians() << " seen=" << seen << " pos=" << getPositionOnLane() << "\\n";\n' + a)
c = '        // adapt to pedestrians on the same lane\n'
assert s.count(c) == 1, ("anchor c", s.count(c))
s = s.replace(c, '        if (getID() == "L1f.0") std::cerr << "PT03 pre t=" << SIMTIME << " lane=" << lane->getID() << " persons=" << lane->getEdge().getPersons().size() << " speed=" << getSpeed() << "\\n";\n' + c)
b = '            if (leader.first != 0) {\n                const double stopSpeed = cfModel.stopSpeed(this, getSpeed(), leader.second - getVehicleType().getMinGap());\n'
assert s.count(b) == 1, ("anchor b", s.count(b))
s = s.replace(b, '            if (getID() == "L1f.0") std::cerr << "PT03 leader_null=" << (leader.first == 0) << " dist=" << leader.second << "\\n";\n' + b)
open(p, 'w').write(s)
q = sys.argv[2]
s = open(q).read()
e = '    for (const MSPModel_InteractingState* ped : getPedestrians(lane)) {\n        // account for distance covered by oncoming pedestrians\n'
assert s.count(e) == 1, ("anchor e", s.count(e))
s = s.replace(e, e + '        if (lane->getID() == "E1_0") std::cerr << "PT03ped edgePos=" << ped->getEdgePos(0) << " lat=" << ped->getLatOffset() << " dir=" << ped->getDirection() << " minPos=" << minPos << " minRight=" << minRight << " maxLeft=" << maxLeft << " bidi=" << bidi << " laneW=" << lane->getWidth() << "\\n";\n')
open(q, 'w').write(s)
PY
git -C $P diff > $T/cpp.patch; echo "patch lines: $(wc -l < $T/cpp.patch)"
CPPBIN=$(bash tools/gcloud/vm_cpp_trace.sh pt03 $T/cpp.patch 2>$T/build.log | tail -1); echo "cpp bin: $CPPBIN"
[ -x "$CPPBIN" ] || { echo "CPP BUILD FAILED:"; tail -12 $T/build.log | cut -c1-240; }
echo "PT03 markers in cpp bin: $(grep -c PT03 $CPPBIN) ; build.log lines: $(wc -l < $T/build.log)"; ls -l --time-style=+%T $CPPBIN | cut -c1-120
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
echo "RS trace lines: $(grep -c PT03 $T/rs/trace.txt) hasPed=true: $(grep -c 'hasPed=true' $T/rs/trace.txt) leader rows: $(grep -c leader_null $T/rs/trace.txt)"
sed -n '/t=28000 lane=E1_0/,/t=29000 lane=E0_0/p' $T/rs/trace.txt | cut -c1-220 | head -12
echo "C++ PT03ped rows: $(grep -c PT03ped $T/cpp/trace.txt)"; grep PT03ped $T/cpp/trace.txt | head -4 | cut -c1-220
echo "C++ pre rows: $(grep -c 'PT03 pre' $T/cpp/trace.txt)"; grep 'PT03' $T/cpp/trace.txt | sed -n '1,4p;/t=26\.00/,/t=30\.00/p' | cut -c1-200 | head -30
echo "personinfo rows ref=$(grep -c '<personinfo' $T/ref/personinfo.xml 2>/dev/null) rs=$(grep -c '<personinfo' $T/rs/personinfo.xml 2>/dev/null)"
