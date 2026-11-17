#!/bin/bash
# rc02 trace (removed in the final commit): first differing rerouter decision, ours vs instrumented C++.
set -u
C=tools/oracle/cases/rerouter_parking
OUT=/tmp/rc02; mkdir -p "$OUT"
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug
# the VM's official tree is at another revision: vm_cpp_trace.sh copies it and resets src, then its git apply
# of an empty patch fails by design; we instrument the copy ourselves by anchoring on single lines.
: > "$OUT/none.patch"
bash tools/gcloud/vm_cpp_trace.sh rc02 "$OUT/none.patch" >/dev/null 2>&1
D=/home/gate/cpp-rc02
[ -d "$D/build" ] || { echo "RC02 no cpp tree at $D"; exit 1; }
(cd "$D" && git checkout -q -- src)
command -v python3 >/dev/null || { echo "RC02 no python3"; exit 1; }
python3 - "$D" <<'PY'
import sys
d = sys.argv[1]
def ins(path, prefix, text, after=True):
    p = f"{d}/src/microsim/trigger/{path}"
    lines = open(p).read().split("\n")
    idx = [i for i, l in enumerate(lines) if l.lstrip().startswith(prefix)]
    assert len(idx) == 1, (path, prefix, len(idx))
    i = idx[0]
    new = text.split("\n")
    if after:
        lines[i + 1:i + 1] = new
    else:
        lines[i:i] = new
    open(p, "w").write("\n".join(lines))
def rep(path, prefix, text):
    p = f"{d}/src/microsim/trigger/{path}"
    lines = open(p).read().split("\n")
    idx = [i for i, l in enumerate(lines) if l.lstrip().startswith(prefix)]
    assert len(idx) == 1, (path, prefix, len(idx))
    lines[idx[0]:idx[0] + 1] = text.split("\n")
    open(p, "w").write("\n".join(lines))
SPR = "MSStoppingPlaceRerouter.cpp"
TR = "MSTriggeredRerouter.cpp"
ins(SPR, "const bool destIsFull = destStoppingPlace", r'''    fprintf(stderr, "RRTRACE rsp t=%ld veh=%s dest=%s destVisible=%s ignoreDest=%s stopAnywhere=%d destFull=%s onTheWay=%s destOcc=%.6f\n", (long)SIMSTEP, veh.getID().c_str(), destStoppingPlace == nullptr ? "null" : destStoppingPlace->getID().c_str(), destVisible ? "true" : "false", ignoreDest ? "true" : "false", stopAnywhere, destIsFull ? "true" : "false", onTheWay == nullptr ? "null" : onTheWay->getID().c_str(), destStoppingPlace == nullptr ? -1. : getLastStepStoppingPlaceOccupancy(destStoppingPlace, &veh));''')
ins(SPR, "if (occupancy < getStoppingPlaceCapacity(stoppingPlaceCandidates[i].first))", r'''            fprintf(stderr, "RRTRACE cand %s vis=%s occ=%.6f cap=%.6f\n", stoppingPlaceCandidates[i].first->getID().c_str(), visible ? "true" : "false", occupancy, getStoppingPlaceCapacity(stoppingPlaceCandidates[i].first));''', after=False)
ins(SPR, "double stoppingPlaceCost = getTargetValue(", r'''            fprintf(stderr, "RRTRACE score %s cost=%.6f\n", it->first->getID().c_str(), stoppingPlaceCost);''')
ins(SPR, "const double targetPos = alternative->getLastFreePos(veh);", r'''    fprintf(stderr, "RRTRACE eval %s targetPos=%.6f occ=%.6f\n", alternative->getID().c_str(), targetPos, occupancy);''')
ins(TR, "const MSEdge* lastEdge = tObject.getRerouteDestination();", r'''    fprintf(stderr, "RRTRACE trig t=%ld veh=%s rr=%s park=%.6f\n", (long)now, tObject.getID().c_str(), getID().c_str(), rerouteDef->parkProbs.getOverallProb());''')
rep(TR, "return dynamic_cast<MSParkingArea*>(rerouteStoppingPlace(destStoppingPlace", r'''    MSStoppingPlace* resultSp = rerouteStoppingPlace(destStoppingPlace, parks, rerouteDef->parkProbs.getProbs(), veh, newDestination, newRoute, addInput, rerouteDef->getClosed());
    fprintf(stderr, "RRTRACE park dest=%s parks=%d result=%s\n", destStoppingPlace->getID().c_str(), (int)parks.size(), resultSp == nullptr ? "null" : resultSp->getID().c_str());
    return dynamic_cast<MSParkingArea*>(resultSp);''')
print("instrumented")
PY
(cd "$D/build" && make -j8 sumo > "$OUT/make.log" 2>&1; echo "RC02 make rc=$?"; tail -3 "$OUT/make.log")
echo "RC02 patched: $(grep -c RRTRACE "$D/src/microsim/trigger/MSTriggeredRerouter.cpp") trig, $(grep -c RRTRACE "$D/src/microsim/trigger/MSStoppingPlaceRerouter.cpp") spr"
ls -l "$D/bin/sumo" | head -2
echo "RC02 binary strings: $(strings "$D/bin/sumo" | grep -c RRTRACE)"
find "$D/build" -name 'MSTriggeredRerouter*' -newer "$OUT/none.patch" | head -5
echo "RC02 objs with RRTRACE: $(grep -rl RRTRACE "$D/build" --include='*.o' 2>/dev/null | head -3)"
grep -c RRTRACE "$D/src/microsim/trigger/MSTriggeredRerouter.cpp"; grep -n 'RRTRACE trig' "$D/src/microsim/trigger/MSTriggeredRerouter.cpp" | head -2
CPP="$D/bin/sumo"
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true --end 400 --additional-files $C/parking.add.xml"
RR_TRACE=1 timeout 300 "$BIN/sumo" $ARGS --tripinfo-output "$OUT/ours.ti.xml" > "$OUT/ours.log" 2>&1
timeout 300 "$CPP" $ARGS --tripinfo-output "$OUT/cpp.ti.xml" > "$OUT/cpp.log" 2>&1
grep -a RRTRACE "$OUT/ours.log" > "$OUT/ours.tr"
grep -a RRTRACE "$OUT/cpp.log" > "$OUT/cpp.tr"
echo "RC02 lines ours=$(wc -l < "$OUT/ours.tr") cpp=$(wc -l < "$OUT/cpp.tr")"
echo "RC02 --- cpp log head"; head -3 "$OUT/cpp.log"
echo "RC02 --- cpp trace (head 40)"; head -40 "$OUT/cpp.tr"
echo "RC02 --- ours trace (head 40)"; head -40 "$OUT/ours.tr"
echo "RC02 --- first diff"; diff "$OUT/cpp.tr" "$OUT/ours.tr" | head -20
echo "RC02 --- tripinfo v2 ours"; grep -a 'id="v2"' "$OUT/ours.ti.xml" | head -1
echo "RC02 --- tripinfo v2 cpp"; grep -a 'id="v2"' "$OUT/cpp.ti.xml" | head -1
echo "RC02 --- errors"; grep -aiE 'error|panic' "$OUT/ours.log" | head -5; grep -aiE 'error|panic' "$OUT/cpp.log" | head -5
echo "RC02 DONE"
