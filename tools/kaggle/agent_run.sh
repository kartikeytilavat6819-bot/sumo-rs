#!/bin/bash
# fp01 TEMPORARY: florence first FCD divergence (remove before merge)
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
[ -x "$B.exe" ] && B=$B.exe
cd tools/oracle/cases/city_florence_pedestrians || exit 1
$B -n ref_simple.net.xml -r routes.rou --tripinfo-output /tmp/fl.ti.xml --fcd-output /tmp/fl.fcd.xml --statistic-output /tmp/fl.stat.xml --no-step-log true --end 600 > /tmp/fl.log 2>&1
echo "exit=$?"
python3 - <<'EOF'
import re
def steps(path):
    out = []; t = None
    for line in open(path):
        m = re.search(r'<timestep time="([^"]+)"', line)
        if m:
            t = m.group(1); out.append((t, {})); continue
        m = re.search(r'<(vehicle|person) id="([^"]+)"', line)
        if m and out:
            out[-1][1][m.group(1) + ":" + m.group(2)] = line.strip()
    return out
ref = steps("ref.fcd.xml"); cand = steps("/tmp/fl.fcd.xml")
print("steps", len(ref), len(cand))
n = 0
for (t, rv), (t2, cv) in zip(ref, cand):
    if t != t2:
        print("time mismatch", t, t2); break
    for k in sorted(set(rv) | set(cv)):
        if rv.get(k) != cv.get(k):
            n += 1
            if n <= 6:
                print("t=%s %s\n  ref : %s\n  cand: %s" % (t, k, rv.get(k), cv.get(k)))
    if n > 6:
        break
print("diffs shown/total:", min(n, 6), n)
EOF
