#!/bin/bash
# oc107 diagnostic: locate the first divergence for the two failing platoon_unstable rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc107_diag
mkdir -p "$OUT"
RUST="$CARGO_TARGET_DIR/debug/sumo.exe"
[ -x "$RUST" ] || RUST="$CARGO_TARGET_DIR/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"

for c in tt_cf_model_ACC_platoon_unstable tt_cf_model_CACC_platoon_unstable; do
  D=tools/oracle/cases/$c
  FLAGS=$(cat "$D/sim.args")
  COMMON="-n $D/ref_simple.net.xml -r $D/routes.rou $FLAGS --end 120 --no-step-log true"
  $OFF $COMMON --tripinfo-output "$OUT/$c.off.ti.xml" --fcd-output "$OUT/$c.off.fcd.xml" > "$OUT/$c.off.log" 2>&1
  echo "$c official rc=$?"
  $RUST $COMMON --tripinfo-output "$OUT/$c.rs.ti.xml" --fcd-output "$OUT/$c.rs.fcd.xml" > "$OUT/$c.rs.log" 2>&1
  echo "$c rust rc=$?"
  echo "--- $c warnings official ---"
  grep -iE 'teleport|collision|emergency|Error' "$OUT/$c.off.log" | head -20 || true
  echo "--- $c warnings rust ---"
  grep -iE 'teleport|collision|emergency|Error|panicked' "$OUT/$c.rs.log" | head -20 || true
done

python3 - <<'PY'
import xml.etree.ElementTree as ET, os
OUT='/tmp/oc107_diag'
def fcd(p):
    d={}
    for e in ET.parse(p).getroot().iter('timestep'):
        t=float(e.get('time'))
        for v in e.iter('vehicle'):
            d[(t,v.get('id'))]=(v.get('x'),v.get('y'),v.get('speed'),v.get('pos'),v.get('lane'))
    return d
for c in ['tt_cf_model_ACC_platoon_unstable','tt_cf_model_CACC_platoon_unstable']:
    a=fcd(f'{OUT}/{c}.off.fcd.xml'); b=fcd(f'{OUT}/{c}.rs.fcd.xml')
    keys=sorted(set(a)&set(b))
    first=None
    for k in keys:
        if a[k]!=b[k]:
            first=(k,a[k],b[k]); break
    print('===',c,'first FCD diff:',first)
    print(' only-off:',sorted({x[1] for x in a}-{x[1] for x in b})[:20])
    print(' only-rs :',sorted({x[1] for x in b}-{x[1] for x in a})[:20])
    # tripinfo ids
    def ti(p):
        return [e.get('id') for e in ET.parse(p).getroot().iter('tripinfo')]
    print(' ti off:',ti(f'{OUT}/{c}.off.ti.xml'))
    print(' ti rs :',ti(f'{OUT}/{c}.rs.ti.xml'))
PY
