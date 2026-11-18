#!/bin/bash
# Gate diagnostic for oc58: full lateral trajectory of the two moped arrival-diff vehicles.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc58-traj
rm -rf "$OUT"; mkdir -p "$OUT"
for spec in "junction_indirect_left_moped flow_0.15" "junction_turnAlignment_moped_indirect_left flow_0.16"; do
  set -- $spec; c=tt_sublane_model_$1; vid=$2
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --fcd-output "$OUT/$c.fcd.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  echo "=== $c ($vid) ==="
  python3 - "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" "$vid" <<'PY'
import sys, xml.etree.ElementTree as ET
def load(p, vid):
    out={}
    for ts in ET.parse(p).getroot().iter('timestep'):
        t=float(ts.get('time'))
        if 135<=t<=160:
            for v in ts.iter('vehicle'):
                if v.get('id')==vid:
                    out[t]=(v.get('x'),v.get('y'),v.get('angle'),v.get('speed'),v.get('pos'),v.get('lane'))
    return out
a,b=load(sys.argv[1],sys.argv[3]),load(sys.argv[2],sys.argv[3])
for t in sorted(set(a)|set(b)):
    ra=' '.join(str(x) for x in a.get(t,('-',)*6))
    rb=' '.join(str(x) for x in b.get(t,('-',)*6))
    mark='  ' if ra==rb else '**'
    print(f"{mark} t={t:6.1f} ref[{ra}] cand[{rb}]")
PY
done
