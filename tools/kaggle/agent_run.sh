#!/bin/bash
# TEMPORARY measurement script (ca03). Removed before the final commit.
set -u
D=tools/oracle/cases/calibrator_multi
O=/tmp/cal_multi
mkdir -p "$O"
OURS="$CARGO_TARGET_DIR/debug/sumo"
REF="$SUMO_PRISTINE_BIN/sumo"
NET="-n $D/ref_simple.net.xml -r $D/routes.rou --no-step-log true --end 200 --additional-files $D/calibrator_multi.add.xml"

CAL_TRACE=1 "$OURS" $NET --tripinfo-output "$O/ours.ti.xml" --fcd-output "$O/ours.fcd.xml" >"$O/ours.log" 2>&1
echo "ours rc=$?"
"$REF" $NET --tripinfo-output "$O/cpp.ti.xml" --fcd-output "$O/cpp.fcd.xml" >"$O/cpp.log" 2>&1
echo "cpp rc=$?"
echo "--- ref vs cpp (tripinfo)"
python tools/oracle/compare_sim.py --tripinfo "$D/ref.tripinfo.xml" "$O/cpp.ti.xml" 2>&1 | tail -3
echo "--- ref vs ours (tripinfo)"
python tools/oracle/compare_sim.py --tripinfo "$D/ref.tripinfo.xml" "$O/ours.ti.xml" 2>&1 | tail -3
echo "--- departs ref vs ours (first 12 diff lines)"
diff <(grep -o '<tripinfo id="[^"]*" depart="[^"]*"' "$D/ref.tripinfo.xml") <(grep -o '<tripinfo id="[^"]*" depart="[^"]*"' "$O/ours.ti.xml") | head -12
echo "--- ours CALTRACE (first 40)"
grep CALTRACE "$O/ours.log" | head -40
echo "--- ours CALTRACE count"
grep -c CALTRACE "$O/ours.log"
echo "--- cpp log tail"
tail -3 "$O/cpp.log"
