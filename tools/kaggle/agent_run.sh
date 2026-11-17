#!/bin/bash
# rx01 measurement only (removed before merge)
set -u
C=tools/oracle/cases/city_german_level_crossing
O=/tmp/rx01; rm -rf $O; mkdir -p $O
run() {
  $1 -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output $O/$2.ti.xml --fcd-output $O/$2.fcd.xml \
     --statistic-output $O/$2.stat.xml --no-step-log true --end 600 > $O/$2.log 2>&1
  echo "$2 rc=$? lines_fcd=$(wc -l < $O/$2.fcd.xml)"
}
run /tmp/target/debug/sumo ours
run $SUMO_PRISTINE_BIN/sumo cpp
cat > $O/cmp.py <<'EOF'
import sys, xml.etree.ElementTree as ET
def load(p):
    d = {}
    for ev, el in ET.iterparse(p):
        if el.tag == 'timestep':
            t = el.get('time')
            for v in el:
                d[(t, v.get('id'))] = tuple(sorted(v.attrib.items()))
            el.clear()
    return d
a = load(sys.argv[1]); b = load(sys.argv[2])
keys = sorted(set(a) | set(b), key=lambda k: (float(k[0]), k[1]))
diffs = [k for k in keys if a.get(k) != b.get(k)]
print('fcd diffs:', len(diffs), 'of', len(keys))
for k in diffs[:6]:
    print('  first', k, '\n    cpp ', b.get(k), '\n    ours', a.get(k))
car = [k for k in diffs if k[1] == 'car81']
print('car81 first diff:', car[:1])
if car:
    print('  cpp ', b.get(car[0])); print('  ours', a.get(car[0]))
EOF
python $O/cmp.py $O/ours.fcd.xml $O/cpp.fcd.xml 2>&1 | head -60
python $O/cmp.py $O/ours.fcd.xml $C/ref.fcd.xml 2>&1 | head -3
grep -c . $O/ours.ti.xml $O/cpp.ti.xml
diff <(grep 'id="car81"' $O/cpp.ti.xml) <(grep 'id="car81"' $O/ours.ti.xml) | head -6
diff $O/cpp.ti.xml $O/ours.ti.xml | head -20
diff $O/cpp.ti.xml $C/ref.tripinfo.xml | head -4
