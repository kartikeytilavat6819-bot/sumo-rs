#!/bin/bash
# jm01 scratch (removed before merge): first FCD difference of fa.10 vs C++ ref, jmparams case
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
for t in sorted(set(r) | set(c), key=float):
    a = r.get(t, {}).get('fa.10'); b = c.get(t, {}).get('fa.10')
    if a != b:
        print('first fa.10 diff at t=', t)
        print(' ref ', a)
        print(' cand', b)
        for k in sorted(set(r.get(t, {})) | set(c.get(t, {}))):
            ra = r.get(t, {}).get(k); ca = c.get(t, {}).get(k)
            if ra != ca:
                print(' veh', k, 'ref', ra and {x: ra[x] for x in ('x','y','speed','angle','lane','pos') if x in ra}, 'cand', ca and {x: ca[x] for x in ('x','y','speed','angle','lane','pos') if x in ca})
        break
else:
    print('fa.10 FCD identical')
EOF
grep -c . /tmp/jm01_run.log
