#!/bin/bash
# Gate diagnostic for oc55: measure every imported tt_extended row.
# Prints first diff line for each failing row (tripinfo / fcd / stat).
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc55-spot
rm -rf "$OUT"; mkdir -p "$OUT"
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_*' | wc -l)
active=$(find tools/oracle/cases -maxdepth 2 -type f -name PENDING -path '*tt_*' | wc -l)
ext=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_extended*' | wc -l)
echo "tt_total=$total tt_pending=$active tt_active=$((total-active)) tt_extended=$ext"

ok=0; bad=0
for d in tools/oracle/cases/tt_extended*/; do
  [ -f "$d/routes.rou" ] || continue
  [ -f "$d/ref.tripinfo.xml" ] || continue
  c=$(basename "$d")
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  if [ "$rc" -ne 0 ]; then
    printf 'FAIL %-64s rc=%s %s\n' "$c" "$rc" "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-60)"
    bad=$((bad+1)); continue
  fi
  r="PASS"; dd=""
  x=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
  [ -n "$x" ] && { r="FAIL"; dd="ti:$x"; }
  if [ -f "$d/ref.fcd.xml" ]; then
    x=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
    [ -n "$x" ] && { r="FAIL"; dd="$dd fcd:$x"; }
  fi
  if [ -f "$d/ref.stat.xml" ]; then
    x=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" 2>&1 | head -1)
    [ -n "$x" ] && { r="FAIL"; dd="$dd stat:$x"; }
  fi
  if [ "$r" = PASS ]; then ok=$((ok+1)); else printf 'FAIL %-64s %s\n' "$c" "$(echo "$dd" | cut -c1-180)"; bad=$((bad+1)); fi
done
echo "EXTENDED ok=$ok bad=$bad"
