#!/bin/bash
# pd02 measurement (remove before merge): ours vs pristine for the pedestrian cases.
cd "$(dirname "$0")/../.." || exit 1
B=/tmp/target/debug
O=/tmp/pd02_out
mkdir -p "$O"
for c in persons peds_crossing; do
  d=tools/oracle/cases/$c
  if [ -f "$d/input.osm" ]; then
    "$B/netconvert" --osm-files "$d/input.osm" --simple-projection $(cat "$d/net.args") -o "$O/$c.net.xml" >"$O/$c.nc.log" 2>&1
  else
    "$B/netconvert" $(cat "$d/net.args") -o "$O/$c.net.xml" >"$O/$c.nc.log" 2>&1
  fi
  python tools/oracle/compare_net.py "$d/ref_simple.net.xml" "$O/$c.net.xml" | grep SUMMARY | head -1 | sed "s/^/$c net: /"
  "$B/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --tripinfo-output "$O/$c.ti.xml" --no-step-log true >"$O/$c.sumo.log" 2>&1
  echo "$c sim exit $? : $(grep -m1 -E 'panicked|Error' "$O/$c.sumo.log" | cut -c1-160)"
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$O/$c.ti.xml" 2>&1 | head -3 | sed "s/^/$c tripinfo: /"
  echo "--- personinfo diff (ref < > ours), first 30 lines:"
  diff <(grep -A2 personinfo "$d/ref.tripinfo.xml") <(grep -A2 personinfo "$O/$c.ti.xml") | head -30
done
