#!/usr/bin/env python3
# oc97 probe (temporary): side-by-side per-step speed/pos/lane of vehicles from two fcd files.
# usage: oc97_cmp.py <ref.fcd.xml> <ours.fcd.xml> <first-step> <last-step> <veh> [<veh> ...]
import re, sys

def load(path):
    steps = {}
    t = None
    rx_t = re.compile(r'<timestep time="([^"]*)"')
    rx_v = re.compile(r'<vehicle id="([^"]*)".* speed="([^"]*)" pos="([^"]*)" lane="([^"]*)"')
    with open(path, errors="replace") as f:
        for line in f:
            m = rx_t.search(line)
            if m:
                t = float(m.group(1))
                continue
            m = rx_v.search(line)
            if m and t is not None:
                steps[(t, m.group(1))] = (m.group(2), m.group(3), m.group(4))
    return steps

ref, ours = load(sys.argv[1]), load(sys.argv[2])
t0, t1 = float(sys.argv[3]), float(sys.argv[4])
for veh in sys.argv[5:]:
    print("-- %s (t: ref speed pos lane | ours speed pos lane), first %d differing rows" % (veh, 40))
    n = 0
    times = sorted({t for (t, v) in list(ref) + list(ours) if v == veh and t0 <= t <= t1})
    for t in times:
        a, b = ref.get((t, veh)), ours.get((t, veh))
        if a != b:
            print("t=%6.1f ref=%s | ours=%s" % (t, a, b))
            n += 1
            if n >= 40:
                break
    if n == 0:
        print("no difference in [%s, %s]" % (t0, t1))
