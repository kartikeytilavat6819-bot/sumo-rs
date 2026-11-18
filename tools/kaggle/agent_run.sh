#!/bin/bash
# oc42 shortcut-audit measurement: run in-scope failing rows and print first diffs vs committed refs.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc42
mkdir -p "$OUT"
CASES="out_bt out_meandata elechybrid_overhead md_midinterval md_noperiod md_persons md_public md_stop md_vtypes md_window"
for c in $CASES; do
  d="tools/oracle/cases/$c"
  [ -f "$d/routes.rou" ] || continue
  EXTRA=""; [ -f "$d/sim.args" ] && EXTRA=$(cat "$d/sim.args")
  XOUT=""
  [ -f "$d/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; XOUT="$XOUT $opt $OUT/$c.$suf.xml"; done < "$d/extra_outputs.txt"
  echo "=== $c ==="
  timeout 300 "$BIN/sumo.exe" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA $XOUT \
      >"$OUT/$c.log" 2>&1
  rc=$?
  echo "exit=$rc"
  [ $rc -ne 0 ] && { grep -m1 -E 'panicked|Error' "$OUT/$c.log"; continue; }
  python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -3
  [ -f "$d/ref.fcd.xml" ] && python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | head -3
  [ -f "$d/ref.stat.xml" ] && python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/$c.stat.xml" 2>&1 | head -3
  [ -f "$d/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; [ -f "$d/ref.$suf.xml" ] || continue
    echo "--- $suf ---"
    python tools/oracle/compare_xml.py "$d/ref.$suf.xml" "$OUT/$c.$suf.xml" 2>&1 | head -4
  done < "$d/extra_outputs.txt"
  [ -f "$d/add_outputs.txt" ] && while read -r af suf; do af=${af%$'\r'}; suf=${suf%$'\r'}; [ -f "$d/ref.$suf.xml" ] || continue
    echo "--- add:$suf ---"
    python tools/oracle/compare_xml.py "$d/ref.$suf.xml" "$d/$af" 2>&1 | head -4
  done < "$d/add_outputs.txt"
done
