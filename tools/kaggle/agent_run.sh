#!/bin/bash
# nc05 measurement, branch-only: must be removed before the final commit.
C=tools/oracle/cases/city_munich
J=1413262637
mkdir -p /tmp/nc05 && cd /tmp/nc05
/tmp/target/debug/netconvert --osm-files /tmp/src/$C/input.osm --simple-projection -o ours.net.xml > ours.out 2> ours.err
echo "NC05 ours rc=$?"
for f in /tmp/src/$C/ref_simple.net.xml ours.net.xml; do
  echo "== $f"
  JL=$(grep "<junction id=\"$J\"" $f)
  echo "$JL" | cut -c1-260
  for lane in $(echo "$JL" | grep -o 'incLanes="[^"]*"' | sed 's/incLanes="//;s/"//' | tr ' ' '\n' | grep -v '^$'); do
    edge=${lane%_*}
    grep -m1 "<edge id=\"$edge\"" $f | cut -c1-200
    grep -m1 "<lane id=\"$lane\"" $f | cut -c1-200
  done
  echo "rail_signal junctions:"
  grep -o 'junction id="[^"]*" type="rail_signal"' $f | sed 's/junction id=//' | tr '\n' ' '
  echo
  echo "rail_crossing junctions: $(grep -c 'type="rail_crossing"' $f)"
done
python3 /tmp/src/tools/oracle/compare_net.py /tmp/src/$C/ref_simple.net.xml ours.net.xml 2>&1 | head -30
