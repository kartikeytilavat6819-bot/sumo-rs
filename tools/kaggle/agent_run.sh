#!/bin/bash
# rc02 trace (removed in the final commit): first differing rerouter decision, ours vs instrumented C++.
set -u
C=tools/oracle/cases/rerouter_parking
OUT=/tmp/rc02; mkdir -p "$OUT"
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug
CPP=$(bash tools/gcloud/vm_cpp_trace.sh rc02 tools/oracle/trace/rc02_cpp_trace.patch) || { echo "RC02 cpp build failed"; exit 1; }
echo "RC02 cpp=$CPP ours=$BIN/sumo"
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true --end 400 --additional-files $C/parking.add.xml"
RR_TRACE=1 timeout 300 "$BIN/sumo" $ARGS --tripinfo-output "$OUT/ours.ti.xml" > "$OUT/ours.log" 2>&1
timeout 300 "$CPP" $ARGS --tripinfo-output "$OUT/cpp.ti.xml" > "$OUT/cpp.log" 2>&1
grep -a RRTRACE "$OUT/ours.log" > "$OUT/ours.tr"
grep -a RRTRACE "$OUT/cpp.log" > "$OUT/cpp.tr"
echo "RC02 lines ours=$(wc -l < "$OUT/ours.tr") cpp=$(wc -l < "$OUT/cpp.tr")"
echo "RC02 --- cpp trace (head 40)"; head -40 "$OUT/cpp.tr"
echo "RC02 --- ours trace (head 40)"; head -40 "$OUT/ours.tr"
echo "RC02 --- first diff"; diff "$OUT/cpp.tr" "$OUT/ours.tr" | head -20
echo "RC02 --- tripinfo v2 ours"; grep -a 'id="v2"' "$OUT/ours.ti.xml" | head -1
echo "RC02 --- tripinfo v2 cpp"; grep -a 'id="v2"' "$OUT/cpp.ti.xml" | head -1
echo "RC02 --- errors"; grep -aiE 'error|panic' "$OUT/ours.log" | head -5; grep -aiE 'error|panic' "$OUT/cpp.log" | head -5
echo "RC02 DONE"
