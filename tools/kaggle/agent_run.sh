#!/bin/bash
# cb01 measurement (remove before final commit)
set -u
for C in tools/oracle/cases/city_paris_dense tools/oracle/cases/city_tokyo_lefthand; do
  echo "===== $C"
  /tmp/target/debug/sumo -n $C/ref_simple.net.xml -r $C/routes.rou --tripinfo-output /tmp/ti.xml --fcd-output /tmp/fcd.xml --statistic-output /tmp/stat.xml --no-step-log true $(cat $C/sim.args) > /tmp/sumo.log 2>&1
  echo "sumo exit=$?"
  python3 - $C/ref.fcd.xml /tmp/fcd.xml <<'EOF'
import sys, re
def load(p):
    out = {}; t = None
    for line in open(p):
        m = re.search(r'<timestep time="([^"]+)"', line)
        if m:
            t = m.group(1); out.setdefault(t, {}); continue
        m = re.search(r'<vehicle id="([^"]+)" ([^/]*)/>', line)
        if m and t is not None:
            out[t][m.group(1)] = m.group(2)
    return out
ref, ours = load(sys.argv[1]), load(sys.argv[2])
ts = sorted(set(ref) | set(ours), key=float)
first = None
for t in ts:
    if ref.get(t, {}) != ours.get(t, {}):
        first = t; break
print("first differing timestep:", first, "| steps ref", len(ref), "ours", len(ours))
if first is not None:
    i = ts.index(first)
    R, O = ref.get(first, {}), ours.get(first, {})
    ids = sorted(set(R) | set(O), key=lambda x: (len(x), x))
    diffs = [v for v in ids if R.get(v) != O.get(v)]
    print("vehicles differing at that step:", len(diffs), diffs[:10])
    for v in diffs[:3]:
        print("  REF ", v, R.get(v))
        print("  OURS", v, O.get(v))
    if i > 0:
        pt = ts[i-1]
        pd = [v for v in set(ref.get(pt, {})) | set(ours.get(pt, {})) if ref.get(pt, {}).get(v) != ours.get(pt, {}).get(v)]
        print("previous step", pt, "differing vehicles:", len(pd))
EOF
done
