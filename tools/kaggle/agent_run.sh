#!/bin/bash
# rx01 measurement only (removed before merge)
set -u
C=tools/oracle/cases/city_german_level_crossing
O=/tmp/rx01; rm -rf $O; mkdir -p $O
CPP=$(bash tools/gcloud/vm_cpp_trace.sh rx01 tools/kaggle/rx01_cpp.patch 2>$O/cppbuild.log | tail -1)
echo "cpp bin: $CPP build-errors: $(grep -c -i error $O/cppbuild.log)"; head -c 1500 $O/cppbuild.log
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
