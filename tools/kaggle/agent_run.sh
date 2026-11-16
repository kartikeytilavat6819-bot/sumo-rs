#!/bin/bash
# Temporary diagnostics for the c10 city cases: first differences, no fixes. Removed before the final commit.
cd "$(dirname "$0")/../.." 2>/dev/null || true
N=/tmp/target/debug/netconvert
S=/tmp/target/debug/sumo
O=/tmp/c10diff
mkdir -p "$O"
for c in city_paris_chapelle_ramps city_amsterdam_bikes; do
  d=tools/oracle/cases/$c
  LH=""
  [ -f "$d/net.args" ] && LH=$(cat "$d/net.args")
  "$N" --osm-files "$d/input.osm" --simple-projection $LH -o "$O/$c.net.xml" > "$O/$c.net.log" 2>&1
  echo "== $c netconvert exit=$? $(grep -m1 -i 'error\|panic' "$O/$c.net.log" | cut -c1-160)"
  if python tools/oracle/compare_net.py "$d/ref_simple.net.xml" "$O/$c.net.xml" > "$O/$c.netcmp.log" 2>&1; then
    echo "NET PASS $c"
  else
    echo "NET FAIL $c"; grep -v '^$' "$O/$c.netcmp.log" | head -6 | cut -c1-220
  fi
  "$S" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$O/$c.ti.xml" --fcd-output "$O/$c.fcd.xml" \
    --statistic-output "$O/$c.stat.xml" --no-step-log true $(cat "$d/sim.args") > "$O/$c.sim.log" 2>&1
  echo "== $c sim exit=$? $(grep -m1 -E 'panicked|Error' "$O/$c.sim.log" | cut -c1-160)"
  r=PASS
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$O/$c.ti.xml" > "$O/$c.ti.log" 2>&1 || r=FAIL
  python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$O/$c.fcd.xml" > "$O/$c.fcd.log" 2>&1 || r=FAIL
  python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$O/$c.stat.xml" > "$O/$c.stat.log" 2>&1 || r=FAIL
  echo "SIM $r $c"
  for f in ti fcd stat; do
    grep -v '^$' "$O/$c.$f.log" | head -3 | sed "s/^/  $f: /" | cut -c1-220
  done
done
