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
strip() { sed -E 's/ devices="[^"]*"//' "$1" | grep '<tripinfo '; }
echo "--- tripinfo ours vs cpp, devices ignored (first 6 diff lines)"
diff <(strip "$O/cpp.ti.xml") <(strip "$O/ours.ti.xml") | head -6
echo "tripinfo rows cpp=$(strip "$O/cpp.ti.xml" | wc -l) ours=$(strip "$O/ours.ti.xml" | wc -l)"
echo "--- tripinfo ref vs cpp, devices ignored (diff line count)"
diff <(strip "$D/ref.tripinfo.xml") <(strip "$O/cpp.ti.xml") | wc -l
echo "--- fcd ours vs cpp"
python tools/oracle/compare_sim.py --fcd "$O/cpp.fcd.xml" "$O/ours.fcd.xml" 2>&1 | tail -2
echo "--- CALTRACE count $(grep -c CALTRACE "$O/ours.log")"
echo "--- cpp log tail"
tail -2 "$O/cpp.log"
