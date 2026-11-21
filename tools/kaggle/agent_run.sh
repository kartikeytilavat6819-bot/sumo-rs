#!/bin/bash
# oc95 diagnostic: our binary only. Set OC95_TRACE so ms_mean_data.rs prints tracker state.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-main}
export OC95_TRACE=1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/c/Users/karti/Temp/oc95-trace
mkdir -p "$OUT"

for c in md_midinterval md_vtypes md_stop out_bt elechybrid_overhead; do
  D="tools/oracle/cases/$c"
  R="$OUT/$c"; rm -rf "$R"; mkdir -p "$R"
  EXTRA=$(cat "$D/sim.args")
  XOUT=""
  [ -f "$D/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; XOUT="$XOUT $opt $R/x.$suf.xml"; done < "$D/extra_outputs.txt"
  [ -f "$D/add_outputs.txt" ] && while read -r af suf; do af=${af%$'\r'}; rm -f "$D/$af"; done < "$D/add_outputs.txt"
  echo "===== CASE $c ====="
  timeout 300 "$BIN/sumo.exe" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" \
      --tripinfo-output "$R/ti.xml" --fcd-output "$R/fcd.xml" \
      --statistic-output "$R/stat.xml" --no-step-log true $EXTRA $XOUT \
      >"$R/sumo.log" 2>&1
  echo "rc=$?  stderr-lines=$(wc -l < "$R/sumo.log")"
  echo "--- TRACE (first 120 lines) ---"
  grep '^OC95' "$R/sumo.log" | head -120
  echo "--- TRACE (last 60 lines) ---"
  grep '^OC95' "$R/sumo.log" | tail -60
  echo "--- DIFF meandata/bt/elec vs ref ---"
  if [ -f "$D/add_outputs.txt" ]; then
    while read -r af suf; do
      af=${af%$'\r'}; suf=${suf%$'\r'}
      [ -f "$D/ref.$suf.xml" ] || continue
      echo "... $suf"
      python tools/oracle/compare_xml.py "$D/ref.$suf.xml" "$D/$af" 2>&1 | head -12
      rm -f "$D/$af"
    done < "$D/add_outputs.txt"
  fi
  for f in "$R"/x.*.xml; do
    [ -f "$f" ] || continue
    suf=${f##*x.}
    [ -f "$D/ref.$suf.xml" ] || continue
    echo "... $suf"
    python tools/oracle/compare_xml.py "$D/ref.$suf.xml" "$f" 2>&1 | head -12
  done
done
