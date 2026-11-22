#!/bin/bash
# oc105 measurement (kept) + oc103 taxi idling diff diagnostic.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc105
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_devices_ssm_bugs_ticket13918 tt_devices_rerouting_reroute_congestions_exit_times; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"
    echo "$side exit=$?"
    echo "$side stderr:"; head -20 "$OUT/$c.$side.err"
    echo "$side stdout:"; head -12 "$OUT/$c.$side.out"
  done
done

echo "==== oc103 taxi idling diff"
T=tools/oracle/cases/tt_devices_taxi_idling_randomCircling_rerouting
TEXTRA=$(cat "$T/sim.args")
timeout 300 "$OUR" -n "$T/ref_simple.net.xml" -r "$T/routes.rou" \
  --tripinfo-output "$OUT/taxi.ti.xml" --fcd-output "$OUT/taxi.fcd.xml" \
  --statistic-output "$OUT/taxi.stat.xml" --no-step-log true $TEXTRA >"$OUT/taxi.log" 2>&1
echo "taxi exit=$?"; head -20 "$OUT/taxi.log"
echo "-- tripinfo diff --"
python tools/oracle/compare_sim.py --tripinfo "$T/ref.tripinfo.xml" "$OUT/taxi.ti.xml" 2>&1 | head -20
echo "-- fcd diff --"
python tools/oracle/compare_sim.py --fcd "$T/ref.fcd.xml" "$OUT/taxi.fcd.xml" 2>&1 | head -20
echo "-- all fcd x diffs --"
python - "$T/ref.fcd.xml" "$OUT/taxi.fcd.xml" <<'PY'
import sys, xml.etree.ElementTree as ET
a = ET.parse(sys.argv[1]).getroot(); b = ET.parse(sys.argv[2]).getroot()
sa = {tim.attrib['time']: tim for tim in a.iter('timestep')}
sb = {tim.attrib['time']: tim for tim in b.iter('timestep')}
n = 0
for t in sorted(set(sa) & set(sb)):
    va = {v.attrib['id']: v.attrib for v in sa[t].iter('vehicle')}
    vb = {v.attrib['id']: v.attrib for v in sb[t].iter('vehicle')}
    for vid in set(va) & set(vb):
        for k in ('x','y','angle','speed','pos','lane'):
            if va[vid].get(k) != vb[vid].get(k):
                print(t, vid, k, 'ref=' + str(va[vid].get(k)), 'cand=' + str(vb[vid].get(k)))
                n += 1
                if n >= 30: sys.exit(0)
PY
echo "-- stat diff --"
python tools/oracle/compare_stat.py "$T/ref.stat.xml" "$OUT/taxi.stat.xml" 2>&1 | head -20
echo "=== AGENT RUN DONE ==="
