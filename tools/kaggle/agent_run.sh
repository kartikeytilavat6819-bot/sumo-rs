#!/bin/bash
# oc95 diagnostic: our binary only. Set OC95_TRACE for tracker/BT state. Linux gate.
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}
export OC95_TRACE=1
BIN="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc95
rm -rf "$OUT"; mkdir -p "$OUT"

for c in md_midinterval md_vtypes md_stop out_bt elechybrid_overhead; do
  D="tools/oracle/cases/$c"
  R="$OUT/$c"; mkdir -p "$R"
  EXTRA=$(cat "$D/sim.args")
  XOUT=""
  [ -f "$D/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; XOUT="$XOUT $opt $R/x.$suf.xml"; done < "$D/extra_outputs.txt"
  [ -f "$D/add_outputs.txt" ] && while read -r af suf; do af=${af%$'\r'}; rm -f "$D/$af" "$R/$af" "$af"; done < "$D/add_outputs.txt"
  echo "===== CASE $c ====="
  timeout 300 "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" \
      --tripinfo-output "$R/ti.xml" --fcd-output "$R/fcd.xml" \
      --statistic-output "$R/stat.xml" --no-step-log true $EXTRA $XOUT \
      >"$R/sumo.log" 2>&1
  echo "rc=$?  stderr-lines=$(wc -l < "$R/sumo.log")"
  grep -m1 -E 'panicked at|Error:' "$R/sumo.log" || true
  echo "--- TRACE (first 80) ---"
  grep '^OC95' "$R/sumo.log" | head -80
  echo "--- TRACE (last 80) ---"
  grep '^OC95' "$R/sumo.log" | tail -80
  if [ "$c" = out_bt ]; then
    echo "--- BT SEEN counts ---"
    grep '^OC95BT SEEN' "$R/sumo.log" | head -200
    echo "--- BT OUTPUT lines ---"
    grep '^OC95BT OUTPUT' "$R/sumo.log" | head -80
    echo "--- BT TRACE (t 4..9) ---"
    grep '^OC95BT' "$R/sumo.log" | awk '{for(i=1;i<=NF;i++) if($i ~ /^step=/){split($i,a,"="); if(a[2]>=4 && a[2]<=9) print}}' | head -80
    echo "--- OUR bt.xml (first 60) ---"
    head -60 "$R/x.bt.xml" 2>/dev/null || echo MISSING
  fi
  echo "--- DIFF meandata/bt/elec vs ref ---"
  if [ -f "$D/add_outputs.txt" ]; then
    while read -r af suf; do
      af=${af%$'\r'}; suf=${suf%$'\r'}
      [ -f "$D/ref.$suf.xml" ] || continue
      f=""
      [ -f "$D/$af" ] && f="$D/$af"
      [ -z "$f" ] && [ -f "$R/$af" ] && f="$R/$af"
      [ -z "$f" ] && [ -f "$af" ] && f="$af"
      echo "... $suf  (found: ${f:-NONE})"
      [ -n "$f" ] && python tools/oracle/compare_xml.py "$D/ref.$suf.xml" "$f" 2>&1 | head -12
      rm -f "$D/$af" "$R/$af" "$af"
    done < "$D/add_outputs.txt"
  fi
  for f in "$R"/x.*.xml; do
    [ -f "$f" ] || continue
    suf=${f##*x.}; suf=${suf%.xml}
    [ -f "$D/ref.$suf.xml" ] || continue
    echo "... $suf"
    python tools/oracle/compare_xml.py "$D/ref.$suf.xml" "$f" 2>&1 | head -12
  done
done
