#!/bin/bash
# ir01 measurement (remove before merge): persons case, ours vs C++ tripinfo (PENDING is ignored here).
cd "$(dirname "$0")/../.." || exit 1
B=/tmp/target/debug
O=/tmp/ir01_out
mkdir -p "$O"
d=tools/oracle/cases/persons
"$B/netconvert" --osm-files "$d/input.osm" --simple-projection $(cat "$d/net.args") -o "$O/persons.net.xml" >"$O/nc.log" 2>&1
echo "netconvert exit $?"
RUST_BACKTRACE=1 "$B/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --tripinfo-output "$O/ti.xml" --no-step-log true >"$O/sumo.log" 2>&1
echo "sim exit $? : $(grep -m1 -E 'panicked|Error' "$O/sumo.log" | cut -c1-200)"
grep -a 'SFDBG' "$O/sumo.log" | sort | uniq -c | head -12 | cut -c1-220
grep -a -m4 -E 'speedFactor=' "$O/ti.xml" | cut -c1-160
grep -a -m4 -E 'personinfo id' "$d/ref.tripinfo.xml" | cut -c1-160
python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$O/ti.xml" 2>&1 | head -3 | sed "s/^/persons tripinfo: /"
diff <(grep -A2 personinfo "$d/ref.tripinfo.xml") <(grep -A2 personinfo "$O/ti.xml") | head -30
