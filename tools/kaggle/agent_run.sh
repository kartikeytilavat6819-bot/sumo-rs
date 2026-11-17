#!/bin/bash
# TEMPORARY agent run (bengaluru veh 115 save/load). Removed before the final commit.
set -u
OUT=/tmp/blr
mkdir -p "$OUT"
B=/tmp/target/debug
d=tools/oracle/cases/city_bengaluru_blr
base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
o=$OUT/ours; mkdir -p "$o"
"$B/sumo" $base --tripinfo-output "$o/straight.ti.xml" --fcd-output "$o/straight.fcd.xml" >"$o/straight.log" 2>&1
echo "straight exit=$?"
"$B/sumo" $base --save-state.times 300 --save-state.files "$o/s.xml" --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
echo "save exit=$?"
"$B/sumo" $base --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" --fcd-output "$o/load.fcd.xml" >"$o/load.log" 2>&1
echo "load exit=$?"
echo "== straight tripinfo 115"; grep 'tripinfo id="115"' $o/straight.ti.xml | cut -c1-260
echo "== load tripinfo 115"; grep 'tripinfo id="115"' $o/load.ti.xml | cut -c1-260
echo "== tripinfo count straight=$(grep -c '<tripinfo ' $o/straight.ti.xml) load=$(grep -c '<tripinfo ' $o/load.ti.xml) save=$(grep -c '<tripinfo ' $o/save.ti.xml)"
echo "== straight fcd 115 (t=300..320)"; grep -A0 'vehicle id="115"' $o/straight.fcd.xml | head -0
python3 - "$o" <<'EOF'
import sys, xml.etree.ElementTree as ET
o = sys.argv[1]
def fcd(p):
    r = {}
    for ts in ET.parse(p).getroot().iter("timestep"):
        t = float(ts.get("time"))
        for v in ts.iter("vehicle"):
            if v.get("id") == "115":
                r[t] = (v.get("lane"), v.get("pos"), v.get("speed"))
    return r
s, l = fcd(o + "/straight.fcd.xml"), fcd(o + "/load.fcd.xml")
print("straight times", min(s) if s else None, max(s) if s else None, "load times", min(l) if l else None, max(l) if l else None)
for t in [299, 300, 301, 302, 305, 310, 320, 340, 400, 500, 556, 557, 558, 599]:
    print(t, "straight", s.get(float(t)), "load", l.get(float(t)))
EOF
echo "== state 115 line (cut)"; grep 'vehicle id="115"' $o/s.xml | cut -c1-900
echo "== load log"; head -c 400 $o/load.log
echo "AGENT RUN DONE"
