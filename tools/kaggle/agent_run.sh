#!/bin/bash
# tx02 trace: taxi0 drop-off, idle and re-insertion, ours vs instrumented C++ (temporary, removed before merge)
OUT=/tmp/tx02
mkdir -p "$OUT"
BIN="$CARGO_TARGET_DIR/debug/sumo"
[ -x "$BIN" ] || BIN="$BIN.exe"
D=/home/gate/cpp-tx02
CPP=""
if [ -d /home/gate/sumo-cpp/bin ]; then
  [ -d "$D" ] || cp -a /home/gate/sumo-cpp "$D"
  (cd "$D" && git checkout -q -- src)
  python3 - "$D" <<'PY' >"$OUT/edit.log" 2>&1
import sys
root = sys.argv[1]
edits = {
 'src/microsim/MSVehicleTransfer.cpp': [(
"""            if (desc.myVeh->getMutableLane()->isInsertionSuccess(desc.myVeh, 0, departPos, desc.myVeh->getLateralPositionOnLane(),
                    false, MSMoveReminder::NOTIFICATION_PARKING)) {
""",
"""            const bool insOk = desc.myVeh->getMutableLane()->isInsertionSuccess(desc.myVeh, 0, departPos, desc.myVeh->getLateralPositionOnLane(),
                    false, MSMoveReminder::NOTIFICATION_PARKING);
            if (desc.myVeh->getID() == "taxi0") {
                std::cerr << "TX02 park t=" << time << " lane=" << desc.myVeh->getMutableLane()->getID() << " pos=" << std::to_string(departPos) << " ok=" << insOk << "\\n";
            }
            if (insOk) {
""")],
 'src/microsim/MSLane.cpp': [(
"""    const double nspeed = safeInsertionSpeed(aVehicle, -pos, leaders, speed);
    if (nspeed == INVALID_SPEED""",
"""    const double nspeed = safeInsertionSpeed(aVehicle, -pos, leaders, speed);
    if (aVehicle->getID() == "taxi0") {
        std::cerr << "TX02 ins t=" << MSNet::getInstance()->getCurrentTimeStep() << " lane=" << getID() << " pos=" << std::to_string(pos) << " speed=" << std::to_string(speed) << " leaders=" << leaders.toString() << " nspeed=" << std::to_string(nspeed) << "\\n";
    }
    if (nspeed == INVALID_SPEED"""), (
"""    const MSLeaderDistanceInfo& followers = getFollowersOnConsecutive(aVehicle, aVehicle->getBackPositionOnLane(), false);
    for (int i = 0; i < followers.numSublanes(); ++i) {
        const MSVehicle* follower = followers[i].first;
        if (follower != nullptr) {
            const double backGapNeeded = follower->getCarFollowModel().getSecureGap(follower, aVehicle, follower->getSpeed(), speed, cfModel.getMaxDecel());
""",
"""    const MSLeaderDistanceInfo& followers = getFollowersOnConsecutive(aVehicle, aVehicle->getBackPositionOnLane(), false);
    if (aVehicle->getID() == "taxi0") {
        std::cerr << "TX02 reachfollow t=" << MSNet::getInstance()->getCurrentTimeStep() << " lane=" << getID() << "\\n";
    }
    for (int i = 0; i < followers.numSublanes(); ++i) {
        const MSVehicle* follower = followers[i].first;
        if (follower != nullptr) {
            const double backGapNeeded = follower->getCarFollowModel().getSecureGap(follower, aVehicle, follower->getSpeed(), speed, cfModel.getMaxDecel());
            if (aVehicle->getID() == "taxi0") {
                std::cerr << "TX02 follower=" << follower->getID() << " gap=" << std::to_string(followers[i].second) << " backGapNeeded=" << std::to_string(backGapNeeded) << "\\n";
            }
""")],
 'src/microsim/devices/MSIdling.cpp': [(
"""    if (!taxi->getHolder().hasStops()) {
""",
"""    if (taxi->getHolder().getID() == "taxi0") {
        std::cerr << "TX02 idle t=" << MSNet::getInstance()->getCurrentTimeStep() << " hasStops=" << taxi->getHolder().hasStops() << "\\n";
    }
    if (!taxi->getHolder().hasStops()) {
"""), (
"""            taxi->getHolder().addTraciStop(stop, errorOut);
""",
"""            taxi->getHolder().addTraciStop(stop, errorOut);
            if (taxi->getHolder().getID() == "taxi0") {
                std::cerr << "TX02 idle addstop pos=" << std::to_string(stopPos.second) << " err=" << errorOut << "\\n";
            }
""")],
 'src/microsim/devices/MSDevice_Taxi.cpp': [(
"""MSDevice_Taxi::updateMove(const SUMOTime traveltime, const double travelledDist) {
""",
"""MSDevice_Taxi::updateMove(const SUMOTime traveltime, const double travelledDist) {
    if (MSNet::getInstance()->getCurrentTimeStep() >= 160000 && myHolder.getID() == "taxi0") {
        std::cerr << "TX02 upd t=" << MSNet::getInstance()->getCurrentTimeStep() << " state=" << myState << " stops=" << myHolder.getStops().size() << " stopped=" << (myHolder.isStopped() ? "true" : "false") << " serviceEnd=" << myServiceEnd << "\\n";
    }
"""), (
"""MSDevice_Taxi::customerArrived(const MSTransportable* person) {
""",
"""MSDevice_Taxi::customerArrived(const MSTransportable* person) {
    if (myHolder.getID() == "taxi0") {
        std::cerr << "TX02 arrived t=" << MSNet::getInstance()->getCurrentTimeStep() << " state=" << myState << " persons=" << myHolder.getPersonNumber() << " stops=" << myHolder.getStops().size() << "\\n";
    }
""")],
}
for rel, pairs in edits.items():
    path = root + '/' + rel
    data = open(path, 'rb').read()
    crlf = b'\r\n' in data
    text = data.decode().replace('\r\n', '\n')
    for old, new in pairs:
        n = text.count(old)
        print(rel, 'anchor matches:', n)
        if n == 1:
            text = text.replace(old, new)
    if crlf:
        text = text.replace('\n', '\r\n')
    open(path, 'wb').write(text.encode())
PY
  cat "$OUT/edit.log"
  (cd "$D/build" && make -j8 sumo 2>&1 | grep -E ' error|Error ' | head -20)
  [ -x "$D/bin/sumo" ] && CPP="$D/bin/sumo"
fi
echo "== cpp binary: $CPP"
cd tools/oracle/cases/taxi || exit 1
ARGS=$(cat sim.args)
TX02_TRACE=1 "$BIN" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$OUT/ours.ti.xml" --no-step-log true $ARGS >"$OUT/ours.log" 2>&1
echo "== ours exit=$?"
[ -n "$CPP" ] && "$CPP" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$OUT/cpp.ti.xml" --no-step-log true $ARGS >"$OUT/cpp.log" 2>&1
echo "== cpp exit=$?"
echo "-- warnings (ours)"
grep -i 'warning\|error' "$OUT/ours.log" | head -n 20
echo "-- TX02 line counts (ours cpp)"
grep -c 'TX02' "$OUT/ours.log"
grep -c 'TX02' "$OUT/cpp.log" 2>/dev/null
echo "-- first divergence"
diff <(grep 'TX02' "$OUT/ours.log") <(grep 'TX02' "$OUT/cpp.log" 2>/dev/null) | head -n 60
echo "-- ours trace"
grep 'TX02' "$OUT/ours.log" | sed -n '1,200p' | grep -v 'reachfollow'
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-200
grep 'tripinfo id="taxi0"' "$OUT/cpp.ti.xml" 2>/dev/null | cut -c1-200
