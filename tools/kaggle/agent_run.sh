#!/bin/bash
# lc03 TEMPORARY diagnostic (removed before merge): first FCD divergence, ours vs committed pristine ref
set -u
B=$CARGO_TARGET_DIR/debug
for c in sl2015_lat04 sl2015_lat08; do
  d=tools/oracle/cases/$c
  O=/tmp/lc03/$c; mkdir -p $O
  $B/sumo -n $d/ref_simple.net.xml -r $d/routes.rou $(cat $d/sim.args) --fcd-output $O/cand.fcd.xml --tripinfo-output $O/cand.ti.xml --no-step-log true > $O/cand.log 2>&1
  echo "== $c ours rc=$? ; $(grep -ci 'panic' $O/cand.log) panics"
  python3 - "$d/ref.fcd.xml" "$O/cand.fcd.xml" <<'EOF'
import sys, xml.etree.ElementTree as ET
def load(p):
    out = {}
    for ev, el in ET.iterparse(p, events=("end",)):
        if el.tag == "timestep":
            t = el.get("time")
            for v in el.findall("vehicle"):
                out[(t, v.get("id"))] = dict(v.attrib)
            el.clear()
    return out
r = load(sys.argv[1]); c = load(sys.argv[2])
diffs = []
for k in sorted(r, key=lambda k: (float(k[0]), k[1])):
    if k not in c:
        diffs.append((k, "missing in ours")); continue
    a, b = r[k], c[k]
    bad = [f for f in a if f not in b or (a[f] != b[f] and f not in ("id",))]
    bad = [f for f in bad if not (f in a and f in b and a[f] == b[f])]
    if bad:
        diffs.append((k, {f: (a.get(f), b.get(f)) for f in bad}))
print("records ref=%d ours=%d diffs=%d" % (len(r), len(c), len(diffs)))
for k, d in diffs[:12]:
    print(k, d)
EOF
done
