#!/bin/bash
# nc05 measurement, branch-only: must be removed before the final commit.
C=tools/oracle/cases/city_munich
mkdir -p /tmp/nc05 && cd /tmp/nc05
/tmp/target/debug/netconvert --osm-files /tmp/src/$C/input.osm --simple-projection -o ours.net.xml > ours.out 2> ours.err
echo "NC05 ours rc=$?"
for f in /tmp/src/$C/ref_simple.net.xml ours.net.xml; do
  echo "== $f"
  for J in $(grep -o 'junction id="[^"]*" type="rail_signal"' $f | sed 's/junction id="//;s/" type.*//'); do
    JL=$(grep "<junction id=\"$J\"" $f)
    echo "rail_signal $J incLanes: $(echo "$JL" | grep -o 'incLanes="[^"]*"')"
    for lane in $(echo "$JL" | grep -o 'incLanes="[^"]*"' | sed 's/incLanes="//;s/"//'); do
      echo "   $(grep -m1 "<lane id=\"$lane\"" $f | grep -o 'allow="[^"]*"' || echo "$lane allow=ALL")"
    done
  done
  echo "rail_crossing junctions: $(grep -c 'type="rail_crossing"' $f)"
done
python3 /tmp/src/tools/oracle/compare_net.py /tmp/src/$C/ref_simple.net.xml ours.net.xml 2>&1 | head -30
