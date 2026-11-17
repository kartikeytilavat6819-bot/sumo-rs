#!/bin/bash
# TEMPORARY agent run (bengaluru veh 115 save/load divergence). Removed before the final commit.
set -u
OUT=/tmp/blr
mkdir -p "$OUT"
d=tools/oracle/cases/city_bengaluru_blr
base="-n $d/ref_simple.net.xml -r $d/routes.rou --end 600 --no-step-log true --no-warnings true"
for side in ours cpp; do
  if [ $side = ours ]; then B=/tmp/target/debug; else B=$SUMO_PRISTINE_BIN; fi
  o=$OUT/$side; mkdir -p "$o"
  "$B/sumo" $base --tripinfo-output "$o/straight.ti.xml" --fcd-output "$o/straight.fcd.xml" >"$o/straight.log" 2>&1
  echo "$side straight exit=$?"
  "$B/sumo" $base --save-state.times 300 --save-state.files "$o/s.xml" --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
  echo "$side save exit=$?"
  "$B/sumo" $base --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" --fcd-output "$o/load.fcd.xml" >"$o/load.log" 2>&1
  echo "$side load exit=$?"
done
python3 - "$OUT" <<'EOF'
import sys, xml.etree.ElementTree as ET
O = sys.argv[1]
def fcd(p):
    r = {}
    for ts in ET.parse(p).getroot().iter("timestep"):
        t = float(ts.get("time"))
        for v in ts.iter("vehicle"):
            if v.get("id") == "115":
                r[t] = (v.get("lane"), v.get("pos"), v.get("speed"))
    return r
def first_diff(a, b):
    for t in sorted(set(a) | set(b)):
        if a.get(t) != b.get(t):
            return t
    return None
ours_s = fcd(O + "/ours/straight.fcd.xml")
cpp_s = fcd(O + "/cpp/straight.fcd.xml")
ours_l = fcd(O + "/ours/load.fcd.xml")
cpp_l = fcd(O + "/cpp/load.fcd.xml")
print("first t ours_straight != cpp_straight:", first_diff(ours_s, cpp_s))
print("first t ours_load != cpp_load:", first_diff(ours_l, cpp_l))
print("first t ours_load != ours_straight (t>=300):", first_diff({t: v for t, v in ours_s.items() if t >= 300}, {t: v for t, v in ours_l.items()}))
print("first t cpp_load != cpp_straight (t>=300):", first_diff({t: v for t, v in cpp_s.items() if t >= 300}, {t: v for t, v in cpp_l.items()}))
for t in range(400, 501, 5):
    t = float(t)
    print(int(t), "ours_s", ours_s.get(t), "cpp_s", cpp_s.get(t), "| ours_l", ours_l.get(t), "cpp_l", cpp_l.get(t))
EOF
for side in ours cpp; do
  echo "== $side load tripinfo 115: $(grep -c 'tripinfo id="115"' $OUT/$side/load.ti.xml) straight: $(grep -c 'tripinfo id="115"' $OUT/$side/straight.ti.xml)"
done
echo "== state 115 ours"; grep 'vehicle id="115"' $OUT/ours/s.xml | cut -c1-900
echo "== state 115 cpp"; grep 'vehicle id="115"' $OUT/cpp/s.xml | cut -c1-900
echo "AGENT RUN DONE"
