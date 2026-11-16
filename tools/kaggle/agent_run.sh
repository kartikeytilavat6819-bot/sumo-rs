#!/bin/bash
# Temporary measurement script for worktree-agent-bc01. Removed before merge.
set -u
ROOT=$(pwd)
C=$ROOT/tools/oracle/cases/city_barcelona_eixample
O=/tmp/bc01_out
rm -rf "$O"; mkdir -p "$O"
cd "$C" || exit 1
/tmp/target/debug/sumo -n ref_simple.net.xml -r routes.rou --end 600 --no-step-log true \
  --fcd-output "$O/fcd.xml" --tripinfo-output "$O/tripinfo.xml" --statistic-output "$O/stat.xml" \
  > "$O/run.log" 2>&1
echo "sumo exit=$?"
grep -m5 -iE 'panicked|assert|Teleporting' "$O/run.log"
cd "$ROOT" || exit 1
python3 tools/oracle/check_ref_complete.py --end 600 --tripinfo "$O/tripinfo.xml" --fcd "$O/fcd.xml" --stat "$O/stat.xml"
python3 tools/oracle/compare_sim.py --fcd "$C/ref.fcd.xml" "$O/fcd.xml" | head -20
python3 - "$C/ref.fcd.xml" "$O/fcd.xml" <<'EOF'
import sys
import xml.etree.ElementTree as ET

def steps(p):
    return [(float(ts.get('time')), {v.get('id'): dict(v.attrib) for v in ts}) for ts in ET.parse(p).getroot().iter('timestep')]

ref, ours = steps(sys.argv[1]), steps(sys.argv[2])
prev = None
for (tr, vr), (to, vo) in zip(ref, ours):
    if tr != to or vr.keys() != vo.keys() or any(vr[k] != vo[k] for k in vr):
        print(f"first step diff t_ref={tr} t_ours={to}")
        for k in sorted(set(vr) | set(vo)):
            if vr.get(k) != vo.get(k):
                print(f"  vehicle {k}\n    ref ={vr.get(k)}\n    ours={vo.get(k)}")
                break
        if prev is not None:
            pt, pv = prev
            print(f"previous step t={pt} (ref side) for the same vehicle:")
            print(f"  ref ={pv.get(k)}")
        break
    prev = (tr, vr)
else:
    print("no step diff in common range")
EOF
