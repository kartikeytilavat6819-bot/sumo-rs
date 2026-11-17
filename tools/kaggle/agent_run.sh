#!/bin/bash
# rx01 measurement only (removed before merge)
set -u
C=tools/oracle/cases/city_german_level_crossing
O=/tmp/rx01; rm -rf $O; mkdir -p $O
D=/home/gate/cpp-rx01
bash tools/gcloud/vm_cpp_trace.sh rx01 /dev/null > /dev/null 2> $O/cppreset.log || true
F=$D/src/microsim/traffic_lights/MSRailCrossing.cpp
python - "$F" <<'EOF'
import sys
p = sys.argv[1]
s = open(p, 'rb').read().decode()
nl = '\r\n' if '\r\n' in s else '\n'
a = '#include "MSRailCrossing.h"' + nl
assert s.count(a) == 1
s = s.replace(a, a + '#include <iostream>' + nl + '#include <cstdlib>' + nl)
b = '    //if (getID() == "cluster_1088529493_1260626727")'
assert s.count(b) == 1
t = '    if (getenv("RX01") != nullptr) { std::cerr << "RX01 upd t=" << now << " step=" << myStep << " stayRed=" << stayRedUntil << "\\n"; for (const MSLink* const l : myIncomingRailLinks) for (const auto& it : l->getApproaching()) std::cerr << "RX01  apv " << it.first->getID() << " arr=" << it.second.arrivalTime << " leave=" << it.second.leavingTime << " dist=" << it.second.dist << "\\n"; }' + nl
s = s.replace(b, t + b)
open(p, 'wb').write(s.encode())
print('patched', p)
EOF
(cd $D/build && make -j8 sumo > $O/cppbuild.log 2>&1); CPP=$D/bin/sumo
echo "cpp bin: $CPP build-errors: $(grep -c -i ' error' $O/cppbuild.log)"; grep -i ' error' $O/cppbuild.log | head -c 1200
run() {
  RX01=1 $1 -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $O/$2.ti.xml --fcd-output $O/$2.fcd.xml \
     --statistic-output $O/$2.stat.xml --no-step-log true --end 600 > $O/$2.log 2> $O/$2.trace
  echo "$2 rc=$? trace_lines=$(grep -c RX01 $O/$2.trace)"
}
run /tmp/target/debug/sumo ours
run "$CPP" cpp
win() { awk '/RX01 upd/{split($3,a,"="); p=(a[2]>=150000 && a[2]<=200000)} p' $1; }
win $O/cpp.trace > $O/cpp.win; win $O/ours.trace > $O/ours.win
echo "windowed upd lines cpp=$(grep -c 'upd' $O/cpp.win) ours=$(grep -c 'upd' $O/ours.win)"
diff $O/cpp.win $O/ours.win | head -60
echo "--- cpp window head"; head -30 $O/cpp.win
