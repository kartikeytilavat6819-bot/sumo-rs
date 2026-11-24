#!/usr/bin/env bash
# oc122 probe (temporary): polyconvert OSM rows, official vs ours
OUT=/tmp/oc122; mkdir -p "$OUT"
for c in polyconvert_itt_polyconvert_import_OSM_inner_shape polyconvert_itt_polyconvert_import_OSM_water_incomplete polyconvert_itt_polyconvert_import_OSM_buildings_from_relation; do
  d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/polyconvert; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/polyconvert
    mkdir -p "$OUT/$c/$side"
    a=$(sed "s#@DIR@#$OUT/$c/$side#g" $d/args)
    timeout 120 "$bin" $a >"$OUT/$c/$side/stdout" 2>"$OUT/$c/$side/stderr"; echo "== $c $side rc=$?"
  done
  echo "-- diff routes.rou.xml (off vs ours)"
  diff "$OUT/$c/off/routes.rou.xml" "$OUT/$c/ours/routes.rou.xml" | head -n 24 | cut -c1-300
  echo "-- diff stdout"; diff "$OUT/$c/off/stdout" "$OUT/$c/ours/stdout" | head -n 8 | cut -c1-200
  echo "-- diff stderr"; diff "$OUT/$c/off/stderr" "$OUT/$c/ours/stderr" | head -n 8 | cut -c1-200
done
# moped rows: first fcd divergence
for c in tt_sublane_model_junction_indirect_left_moped tt_junction_model_indirect_left_turn_internal_junction_controlled_moped tt_sublane_model_junction_turnAlignment_moped_indirect_left; do
  d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") --fcd-output "$OUT/$c.$side.fcd.xml" --tripinfo-output "$OUT/$c.$side.ti.xml" >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"; echo "== $c $side rc=$?"
  done
  echo "-- first fcd diffs"
  diff "$OUT/$c.off.fcd.xml" "$OUT/$c.ours.fcd.xml" | head -n 12 | cut -c1-220
  echo "-- first diverging vehicle, +-3 steps (off|ours)"
  python3 - "$OUT/$c.off.fcd.xml" "$OUT/$c.ours.fcd.xml" <<'PY'
import re,sys
def load(f):
    d={};t=None
    for l in open(f):
        m=re.search(r'<timestep time="([\d.]+)"',l)
        if m: t=float(m.group(1))
        m=re.search(r'<vehicle id="([^"]+)"',l)
        if m: d.setdefault(m.group(1),{})[t]=re.sub(r'(id|type|slope)="[^"]*" ?','',l.strip())
    return d
a=load(sys.argv[1]);b=load(sys.argv[2]);best=None
for v in a:
    for t in sorted(a[v]):
        if b.get(v,{}).get(t)!=a[v][t]:
            if best is None or t<best[0]: best=(t,v)
            break
if best:
    t,v=best
    for tt in [t-3+i for i in range(7)]:
        print(v,tt,"|",a[v].get(tt),"|",b.get(v,{}).get(tt))
PY
  echo "-- tripinfo diff"
  diff <(grep -o 'id="[^"]*"\|arrival="[^"]*"' "$OUT/$c.off.ti.xml") <(grep -o 'id="[^"]*"\|arrival="[^"]*"' "$OUT/$c.ours.ti.xml") | head -n 8
done
echo "AGENT RUN DONE"
