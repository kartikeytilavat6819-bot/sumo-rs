#!/bin/bash
# Gate diagnostic for oc58: locate first FCD divergence for the two moped arrival-diff rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-spot
rm -rf "$OUT"; mkdir -p "$OUT"
for spec in "junction_indirect_left_moped flow_0.15" "junction_turnAlignment_moped_indirect_left flow_0.16"; do
  set -- $spec; c=tt_sublane_model_$1; vid=$2
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --fcd-output "$OUT/$c.fcd.xml" --tripinfo-output "$OUT/$c.ti.xml" \
      --no-step-log true >"$OUT/$c.log" 2>&1
  echo "=== $c ($vid) rc=$? ==="
  python3 - "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" "$vid" <<'PY'
import sys, xml.etree.ElementTree as ET
ref, cand, vid = sys.argv[1], sys.argv[2], sys.argv[3]
def load(p):
    out = {}
    root = ET.parse(p).getroot()
    for ts in root.iter('timestep'):
        t = ts.get('time')
        for v in ts.iter('vehicle'):
            if v.get('id') == vid:
                out[t] = dict(v.attrib)
    return out
a, b = load(ref), load(cand)
keys = ['x','y','angle','type','speed','pos','lane','posLat','slope']
for t in sorted(set(a)|set(b), key=float):
    if a.get(t) != b.get(t):
        print('t=',t)
        print('  ref ', {k:a.get(t,{}).get(k) for k in keys})
        print('  cand', {k:b.get(t,{}).get(k) for k in keys})
        break
else:
    print('no FCD difference for', vid)
PY
done
