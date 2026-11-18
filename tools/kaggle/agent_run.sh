#!/bin/bash
# oc16 measurement: run lc_lc2013_turnlanes_dur/ref and diff against refs
C=tools/oracle/cases/lc_lc2013_turnlanes_dur
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
OUT=${OUT:-/tmp/oc16_run}
mkdir -p "$OUT"
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
echo "== running our sumo: $BIN"
$BIN -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
    --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
    --statistic-output "$OUT/stat.xml" --no-step-log true \
    --end 150 --lanechange.duration 2 >"$OUT/sumo.log" 2>&1
echo "exit=$?"
if grep -q "panicked at" "$OUT/sumo.log"; then
  echo "== panic header"
  grep -n -A6 "panicked at" "$OUT/sumo.log" | head -20
else
  echo "== tripinfo diff"
  python3 tools/oracle/compare_sim.py --tripinfo "$C/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -20
  echo "== fcd diff"
  python3 tools/oracle/compare_sim.py --fcd "$C/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -20
  echo "== stat diff"
  python3 tools/oracle/compare_stat.py "$C/ref.stat.xml" "$OUT/stat.xml" 2>&1 | head -20
fi
