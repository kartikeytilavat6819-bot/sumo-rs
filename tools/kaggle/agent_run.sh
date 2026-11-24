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
  echo "-- diff routes.rou.xml (off vs ours, header lines excluded)"
  diff <(grep -v "generated on\|output-file value" "$OUT/$c/off/routes.rou.xml") <(grep -v "generated on\|output-file value" "$OUT/$c/ours/routes.rou.xml") | head -n 12 | cut -c1-300
done
echo "AGENT RUN DONE"
