#!/bin/bash
# Temporary first-diff print for failing city rows. Removed before merge.
O=/tmp/first_diff
mkdir -p "$O"
for c in city_london_lh city_kansas_rural city_karlsruhe_tram; do
  d=tools/oracle/cases/$c
  EXTRA=$(cat "$d/sim.args" 2>/dev/null)
  /tmp/target/debug/sumo.exe -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output "$O/$c.ti.xml" --fcd-output "$O/$c.fcd.xml" --statistic-output "$O/$c.stat.xml" \
    --no-step-log true $EXTRA > "$O/$c.log" 2>&1
  echo "== $c sumo exit=$?"
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$O/$c.ti.xml" 2>&1 | head -3
  python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$O/$c.fcd.xml" 2>&1 | head -3
  python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$O/$c.stat.xml" 2>&1 | head -3
done
d=tools/oracle/cases/city_munich
NETEXTRA=$(cat "$d/net.args" 2>/dev/null)
/tmp/target/debug/netconvert.exe --osm-files "$d/input.osm" --simple-projection $NETEXTRA -o "$O/munich.net.xml" > "$O/munich.nc.log" 2>&1
echo "== city_munich netconvert exit=$?"
python tools/oracle/compare_net.py "$d/ref_simple.net.xml" "$O/munich.net.xml" 2>&1 | head -6
