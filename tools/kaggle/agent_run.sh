#!/bin/bash
# rx01 measurement only (removed before merge)
set -u
C=tools/oracle/cases/city_german_level_crossing
O=/tmp/rx01; rm -rf $O; mkdir -p $O
D=/home/gate/cpp-rx01
bash tools/gcloud/vm_cpp_trace.sh rx01 /dev/null > /dev/null 2> $O/cppreset.log || true
F=$D/src/microsim/cfmodels/MSCFModel_Rail.cpp
python - "$F" <<'EOF'
import sys
p = sys.argv[1]
s = open(p, 'rb').read().decode().replace('\r\n', '\n')
def rep(anchor, new, after):
    global s
    assert s.count(anchor) == 1, anchor
    s = s.replace(anchor, anchor + new if after else new + anchor)
rep('#include "MSCFModel_Rail.h"\n', '#include <cstring>\n#include <cstdlib>\n#include <cstdio>\n', True)
rep('MSCFModel_Rail::MSCFModel_Rail(const MSVehicleType* vtype) :', r'''static unsigned long long rx01b(double d) { unsigned long long u; memcpy(&u, &d, 8); return u; }
static bool rx01on(const MSVehicle* veh) { const SUMOTime t = SIMSTEP; return getenv("RX01") != nullptr && veh->getID() == "train3" && t >= 180000 && t <= 185000; }
''', False)
rep('    const double vmax = maxNextSpeed(speed, veh);\n', r'''    if (rx01on(veh)) fprintf(stderr, "RX01 follow t=%lld speed=%llx gap=%llx vsafe=%llx vmin=%llx vmax=%llx\n", (long long)SIMSTEP, rx01b(speed), rx01b(gap), rx01b(vsafe), rx01b(vmin), rx01b(vmax));
''', True)
rep('    const double vMin = speed - ACCEL2SPEED(a);\n', r'''    if (rx01on(veh)) fprintf(stderr, "RX01 min t=%lld speed=%llx res=%llx gr=%llx cr=%llx a=%llx vMin=%llx\n", (long long)SIMSTEP, rx01b(speed), rx01b(res), rx01b(gr), rx01b(cr), rx01b(a), rx01b(vMin));
''', True)
rep('    return MIN2(myTrainParams.vmax, maxNextSpeed);\n', r'''    if (rx01on(veh)) fprintf(stderr, "RX01 max t=%lld speed=%llx res=%llx gr=%llx cr=%llx trac=%llx a=%llx mns=%llx\n", (long long)SIMSTEP, rx01b(speed), rx01b(res), rx01b(gr), rx01b(cr), rx01b(trac), rx01b(a), rx01b(maxNextSpeed));
''', False)
rep('    return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, TS, false), maxNextSpeed(speed, veh));\n', r'''    if (rx01on(veh)) fprintf(stderr, "RX01 stop t=%lld speed=%llx gap=%llx decel=%llx vsafe=%llx mns=%llx\n", (long long)SIMSTEP, rx01b(speed), rx01b(gap), rx01b(decel), rx01b(maximumSafeStopSpeed(gap, decel, speed, false, TS, false)), rx01b(maxNextSpeed(speed, veh)));
''', False)
open(p, 'wb').write(s.encode())
print('patched', p)
EOF
python - "$D/src/microsim/MSVehicle.cpp" <<'EOF'
import sys
p = sys.argv[1]
s = open(p, 'rb').read().decode().replace('\r\n', '\n')
a = 'const bool ignoreRedLink = ignoreRed(link, canBrake) || beyondStopLine;\n'
assert s.count(a) == 1, a
t = r'''            if (getenv("RX01") != nullptr && getID() == "train3" && SIMSTEP >= 180000 && SIMSTEP <= 185000) { union { double d; unsigned long long u; } cvt; cvt.d = dpi.myDistance; unsigned long long dist = cvt.u; cvt.d = dpi.myVLinkWait; unsigned long long vW = cvt.u; cvt.d = dpi.myVLinkPass; unsigned long long vP = cvt.u; cvt.d = myState.mySpeed; unsigned long long sp = cvt.u; fprintf(stderr, "RX01 lnk t=%lld link=%s ls=%d yel=%d cb=%d ign=%d beyond=%d dist=%llx vW=%llx vP=%llx req=%d spd=%llx\n", (long long)SIMSTEP, link->getLaneBefore()->getID().c_str(), (int)ls, (int)yellow, (int)canBrake, (int)ignoreRedLink, (int)beyondStopLine, dist, vW, vP, (int)dpi.mySetRequest, sp); }
'''
s = s.replace(a, a + t)
b = '        if (lastLink != nullptr) {\n            lastLink->adaptLeaveSpeed(vsafeLeader);\n        }\n        v = MIN2(v, vsafeLeader);\n'
assert s.count(b) == 1, 'jl anchor'
u = r'''        if (getenv("RX01") != nullptr && getID() == "train3" && SIMSTEP >= 180000 && SIMSTEP <= 185000) { union { double d; unsigned long long u; } cvt; cvt.d = leaderInfo.second; unsigned long long ld = cvt.u; cvt.d = distToCrossing; unsigned long long dc = cvt.u; cvt.d = vsafeLeader; unsigned long long vs = cvt.u; cvt.d = vLinkPass; unsigned long long vl = cvt.u; cvt.d = v; unsigned long long vv = cvt.u; fprintf(stderr, "RX01 jl t=%lld leaderIsMe=%d leaderDist=%llx distCP=%llx vsafeLeader=%llx vLinkPass=%llx v=%llx\n", (long long)SIMSTEP, (int)(leaderInfo.first == this), ld, dc, vs, vl, vv); }
'''
s = s.replace(b, u + b)
c = '    double vLinkPass = MIN2(cfModel.estimateSpeedAfterDistance(seen, v, cfModel.getMaxAccel()), laneMaxV); // upper bound\n'
assert s.count(c) == 1, 'init anchor'
s = s.replace(c, c + r'''    if (getenv("RX01") != nullptr && getID() == "train3" && SIMSTEP >= 180000 && SIMSTEP <= 185000) { union { double d; unsigned long long u; } cvt; cvt.d = seen; unsigned long long a1 = cvt.u; cvt.d = v; unsigned long long a2 = cvt.u; cvt.d = laneMaxV; unsigned long long a3 = cvt.u; cvt.d = vLinkPass; unsigned long long a4 = cvt.u; fprintf(stderr, "RX01 init t=%lld seen=%llx v=%llx laneMaxV=%llx vLinkPass=%llx\n", (long long)SIMSTEP, a1, a2, a3, a4); }
''')
d = 'const SUMOTime arrivalTime = getArrivalTime(t, seen, v, vLinkPass);\n'
assert s.count(d) == 1, 'fin anchor'
i0 = s.index('lastLink->adaptLeaveSpeed(futureVSafe);')
j = s.index('        v = MIN2(v, vsafeLeader);\n', i0)
s = s[:j] + r'''        if (getenv("RX01") != nullptr && getID() == "train3" && SIMSTEP >= 180000 && SIMSTEP <= 185000) { union { double d; unsigned long long u; } cvt; cvt.d = leaderInfo.second; unsigned long long ld = cvt.u; cvt.d = vsafeLeader; unsigned long long vs = cvt.u; cvt.d = vLinkPass; unsigned long long vl = cvt.u; cvt.d = v; unsigned long long vv = cvt.u; fprintf(stderr, "RX01 adL t=%lld leaderDist=%llx vsafeLeader=%llx vLinkPass=%llx v=%llx\n", (long long)SIMSTEP, ld, vs, vl, vv); }
''' + s[j:]
s = s.replace(d, r'''if (getenv("RX01") != nullptr && getID() == "train3" && SIMSTEP >= 180000 && SIMSTEP <= 185000) { union { double d; unsigned long long u; } cvt; cvt.d = seen; unsigned long long a1 = cvt.u; cvt.d = v; unsigned long long a2 = cvt.u; cvt.d = vLinkPass; unsigned long long a4 = cvt.u; fprintf(stderr, "RX01 fin t=%lld seen=%llx v=%llx vLinkPass=%llx\n", (long long)SIMSTEP, a1, a2, a4); }
        ''' + d)
open(p, 'wb').write(s.encode())
print('patched MSVehicle')
EOF
(cd $D/build && make -j8 sumo > $O/cppbuild.log 2>&1); CPP=$D/bin/sumo
echo "cpp bin: $CPP errors: $(grep -c -i ' error' $O/cppbuild.log) binary RX01: $(grep -a -c RX01 $CPP)"
grep -i ' error' $O/cppbuild.log | head -c 800
run() {
  RX01=1 $1 -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $O/$2.ti.xml --fcd-output $O/$2.fcd.xml \
     --statistic-output $O/$2.stat.xml --no-step-log true --end 190 > $O/$2.log 2> $O/$2.trace
  echo "$2 rc=$? trace_lines=$(grep -c RX01 $O/$2.trace)"
}
run /tmp/target/debug/sumo ours
run "$CPP" cpp
grep RX01 $O/cpp.trace > $O/cpp.rx; grep RX01 $O/ours.trace > $O/ours.rx
echo "first diff line: $(diff $O/cpp.rx $O/ours.rx | head -1)"
echo "--- cpp t=185 all"; grep 'RX01 [a-z]* t=185000' $O/cpp.rx | head -20
echo "--- ours t=185 all"; grep 'RX01 [a-z]* t=185000' $O/ours.rx | head -20
