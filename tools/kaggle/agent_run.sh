#!/bin/bash
# TEMPORARY diagnostic for cy02: first differing quantity for the two failing new city sim rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
B=${CARGO_TARGET_DIR:-/tmp/target}/debug
SUMO=$B/sumo; [ -x "$SUMO" ] || SUMO=$B/sumo.exe
for c in city_berlin_priority city_zurich_tram; do
  d=tools/oracle/cases/$c
  O=/tmp/cy02_$c
  mkdir -p $O
  $SUMO -n $d/ref_simple.net.xml -r $d/routes.rou --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml \
    --statistic-output $O/stat.xml --no-step-log true --end 600 > $O/sumo.log 2>&1
  echo "== $c sumo exit=$?"
  grep -m2 -E 'panicked|Error' $O/sumo.log
  echo "-- tripinfo"; python3 tools/oracle/compare_sim.py --tripinfo $d/ref.tripinfo.xml $O/ti.xml 2>&1 | head -4
  echo "-- fcd";      python3 tools/oracle/compare_sim.py --fcd $d/ref.fcd.xml $O/fcd.xml 2>&1 | head -4
  echo "-- stat";     python3 tools/oracle/compare_stat.py $d/ref.stat.xml $O/stat.xml 2>&1 | head -4
done
