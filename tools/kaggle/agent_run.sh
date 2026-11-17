#!/bin/bash
# jm01 scratch (removed before merge): earliest FCD difference across vehicles, jmparams case
cd tools/oracle/cases/jmparams || exit 1
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
$B -n ref_simple.net.xml -r routes.rou --end 200 --seed 3 --fcd-output /tmp/jm01_fcd.xml \
  --tripinfo-output /tmp/jm01_trip.xml --no-step-log true >/tmp/jm01_run.log 2>&1
echo "sumo exit=$?"
python3 - <<'EOF'
import xml.etree.ElementTree as ET
def load(p):
    out = {}
    for ts in ET.parse(p).getroot().iter('timestep'):
        out[ts.get('time')] = {v.get('id'): v.attrib for v in ts.iter('vehicle')}
    return out
r = load('ref.fcd.xml'); c = load('/tmp/jm01_fcd.xml')
ts = sorted(set(r) | set(c), key=float)
first = None
for t in ts:
    rv, cv = r.get(t, {}), c.get(t, {})
    diffs = sorted(k for k in set(rv) | set(cv) if rv.get(k) != cv.get(k))
    if diffs:
        first = t
        print('earliest diff at t=', t, 'vehicles:', diffs)
        for k in diffs[:3]:
            print(' ', k, 'ref', rv.get(k) and {x: rv[k][x] for x in ('lane','pos','speed') if x in rv[k]}, 'cand', cv.get(k) and {x: cv[k][x] for x in ('lane','pos','speed') if x in cv[k]})
        break
if first is None:
    print('FCD identical')
else:
    for t in ts:
        if float(first) - 8 <= float(t) <= float(first) + 1 and float(t) >= 0:
            a = r.get(t, {}).get('fb.10'); b = c.get(t, {}).get('fb.10')
            print('fb.10 t=', t, 'ref', a and (a['lane'], a['pos'], a['speed']), 'cand', b and (b['lane'], b['pos'], b['speed']))
EOF
grep -c . /tmp/jm01_run.log
