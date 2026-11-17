#!/bin/bash
# TEMPORARY diagnostic for cy04: first differing quantity for the five new city cases (net and sim stages).
set -u
cd "$(dirname "$0")/../.." || exit 1
B=${CARGO_TARGET_DIR:-/tmp/target}/debug
NC=$B/netconvert; [ -x "$NC" ] || NC=$B/netconvert.exe
SUMO=$B/sumo; [ -x "$SUMO" ] || SUMO=$B/sumo.exe
for c in city_paris_etoile city_portland_actuated city_autobahn_interchange city_rotterdam_port city_sf_hills; do
  d=tools/oracle/cases/$c
  O=/tmp/cy04_$c
  mkdir -p $O
  NETEXTRA=""; [ -f $d/net.args ] && NETEXTRA=$(cat $d/net.args)
  $NC --osm-files $d/input.osm --simple-projection $NETEXTRA -o $O/net.xml > $O/net.log 2>&1
  echo "== $c net exit=$?"
  grep -m2 -E 'panicked|Error' $O/net.log
  python3 tools/oracle/compare_net.py $d/ref_simple.net.xml $O/net.xml 2>&1 | head -4
  $SUMO -n $d/ref_simple.net.xml -r $d/routes.rou --tripinfo-output $O/ti.xml --fcd-output $O/fcd.xml \
    --statistic-output $O/stat.xml --no-step-log true $(cat $d/sim.args) > $O/sumo.log 2>&1
  echo "== $c sim exit=$?"
  grep -m2 -E 'panicked|Error' $O/sumo.log
  echo "-- tripinfo"; python3 tools/oracle/compare_sim.py --tripinfo $d/ref.tripinfo.xml $O/ti.xml 2>&1 | head -4
  echo "-- fcd";      python3 tools/oracle/compare_sim.py --fcd $d/ref.fcd.xml $O/fcd.xml 2>&1 | head -4
  echo "-- stat";     python3 tools/oracle/compare_stat.py $d/ref.stat.xml $O/stat.xml 2>&1 | head -4
done
