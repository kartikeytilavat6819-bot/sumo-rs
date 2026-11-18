#!/bin/bash
# oc72 measurement: run 5 failing tt_ cases with our sumo and the pristine sumo.
set +e
OURS=${OURS:-/tmp/target/debug/sumo}
if [ -x "$OURS.exe" ]; then OURS="$OURS.exe"; fi
REF=${SUMO_PRISTINE_BIN}/sumo
C=tools/oracle/cases
OUT=/tmp/oc72measure
mkdir -p "$OUT"

cases="tt_basic_personFlow_walk_routeDist2 tt_basic_person_walking_walk_routeDist tt_basic_routes_parsing_nested_ref tt_basic_routes_parsing_repeat_stops_no_period tt_basic_person_person_inserted_departPos"

for name in $cases; do
  d=$C/$name
  echo "==================== $name ===================="
  extra=$(cat "$d/sim.args" | tr -d '\r')
  echo "--- OURS ---"
  "$OURS" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$name.ti.xml" --fcd-output "$OUT/$name.fcd.xml" \
      --statistic-output "$OUT/$name.stat.xml" --no-step-log true $extra \
      >"$OUT/$name.ours.log" 2>&1
  echo "ours rc=$?"
  tail -12 "$OUT/$name.ours.log"
  echo "--- PRISTINE ---"
  "$REF" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$name.ref.ti.xml" --fcd-output "$OUT/$name.ref.fcd.xml" \
      --statistic-output "$OUT/$name.ref.stat.xml" --no-step-log true $extra \
      >"$OUT/$name.ref.log" 2>&1
  echo "ref rc=$?"
  tail -6 "$OUT/$name.ref.log"
done
echo "AGENT RUN DONE"
