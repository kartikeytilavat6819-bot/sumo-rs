#!/bin/bash
# oc73 measurement: compare our sim output against committed refs for the
# junction/approach-related rows (pointer-ordered container determinism class).
OURS=$CARGO_TARGET_DIR/debug/sumo
[ -x "$OURS" ] || OURS=/tmp/target/debug/sumo
[ -x "$OURS" ] || OURS=/tmp/target/debug/sumo.exe
OUT=/tmp/oc73
rm -rf "$OUT"; mkdir -p "$OUT"
for c in city_delft_campus_crossings city_florence_pedestrians city_german_level_crossing city_grenoble_tram_bus jmparams; do
  d=tools/oracle/cases/$c
  EXTRA=$(cat "$d/sim.args" 2>/dev/null | tr -d '\r')
  "$OURS" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
    --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
    --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA \
    >"$OUT/$c.log" 2>&1
  rc=$?
  echo "== $c (rc=$rc) =="
  if [ $rc -eq 0 ]; then
    python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -2
  else
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" 2>/dev/null | cut -c1-120
  fi
done
