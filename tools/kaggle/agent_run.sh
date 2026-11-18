#!/bin/bash
# oc26: measure meandata diffs between our sumo and pristine C++ sumo.
set -u
OURS=${OURS:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
PRIS=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/oc26
mkdir -p $OUT
CASES=${CASES:-"md_vtypes md_stop md_midinterval"}
for c in $CASES; do
  dir=tools/oracle/cases/$c
  sim=$(cat $dir/sim.args)
  files=""
  while read -r af suf; do
    af=${af%$'\r'}; suf=${suf%$'\r'}
    files="$files $af"
  done < $dir/add_outputs.txt
  for side in ours pris; do
    rm -f $dir/out_*.xml
    if [ $side = ours ]; then BIN=$OURS; else BIN=$PRIS; fi
    $BIN -n $dir/ref_simple.net.xml -r $dir/routes.rou $sim --no-step-log true > $OUT/$c.$side.log 2>&1
    echo "CASE $c side=$side exit=$?"
    for af in $files; do
      if [ -f "$dir/$af" ]; then cp "$dir/$af" "$OUT/$c.$side.$af"; else echo "  MISSING $af"; fi
    done
  done
  for af in $files; do
    if [ -f "$OUT/$c.ours.$af" ] && [ -f "$OUT/$c.pris.$af" ]; then
      echo "  $af: $(python3 tools/oracle/compare_xml.py "$OUT/$c.pris.$af" "$OUT/$c.ours.$af" 2>&1 | head -1)"
      python3 - "$OUT/$c.pris.$af" "$OUT/$c.ours.$af" <<'PY'
import sys, xml.etree.ElementTree as ET
pris, ours = sys.argv[1], sys.argv[2]
for label, path in (("REF", pris), ("CAND", ours)):
    root = ET.parse(path).getroot()
    for iv in root.findall("interval"):
        print(f"  {label} interval {iv.get('begin')}-{iv.get('end')}")
        for e in iv.findall("edge"):
            print(f"    {e.get('id')}: ss={e.get('sampledSeconds')} entered={e.get('entered')} left={e.get('left')} speed={e.get('speed')}")
PY
    fi
  done
done
