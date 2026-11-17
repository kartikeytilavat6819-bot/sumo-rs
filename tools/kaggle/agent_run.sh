#!/bin/bash
# TEMPORARY agent run (bengaluru veh 115 save/load). Removed before the final commit.
set -u
OUT=/tmp/blr
mkdir -p "$OUT"
d=tools/oracle/cases/city_bengaluru_blr
base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
for side in ours cpp; do
  if [ $side = ours ]; then B=/tmp/target/debug; else B=$SUMO_PRISTINE_BIN; fi
  o=$OUT/$side; mkdir -p "$o"
  "$B/sumo" $base --save-state.times 300 --save-state.files "$o/s.xml" --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
  echo "$side save exit=$?"
  "$B/sumo" $base --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" >"$o/load.log" 2>&1
  echo "$side load exit=$?"
  echo "$side state vehicles: $(grep -c '<vehicle ' $o/s.xml)"
  echo "$side state veh115: $(grep -c 'id="115"' $o/s.xml) load tripinfo115: $(grep -c 'tripinfo id="115"' $o/load.ti.xml) load tripinfos: $(grep -c '<tripinfo ' $o/load.ti.xml)"
  head -c 300 $o/load.log
done
grep -o 'vehicle id="[^"]*"' $OUT/ours/s.xml | sort > $OUT/ours.ids
grep -o 'vehicle id="[^"]*"' $OUT/cpp/s.xml | sort > $OUT/cpp.ids
echo "== state id diff (ours vs cpp)"; diff $OUT/cpp.ids $OUT/ours.ids | head -20
echo "== state body diff lines: $(python3 tools/oracle/cases/state_roundtrip/compare.py state $OUT/ours/s.xml $OUT/cpp/s.xml | head -1)"
python3 tools/oracle/cases/state_roundtrip/compare.py state $OUT/ours/s.xml $OUT/cpp/s.xml | grep -n '115' | head -12
echo "== veh115 in cpp save"; grep -n 'id="115"' $OUT/cpp/s.xml | head -3
echo "== veh115 in ours save"; grep -n 'id="115"' $OUT/ours/s.xml | head -3
echo "AGENT RUN DONE"
