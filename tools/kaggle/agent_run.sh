#!/bin/bash
# Gate diagnostic for oc55: measure every imported tt_extended row.
# Prints the first diff / error line for each failing row.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo"
[ -x "$SUMO" ] || SUMO="$BIN/sumo.exe"
OUT="${TMPDIR:-/tmp}/oc55-spot"
rm -rf "$OUT"; mkdir -p "$OUT" || { echo "no temp dir $OUT"; exit 1; }
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_*' | wc -l)
pending=$(find tools/oracle/cases -maxdepth 2 -type f -name PENDING -path '*tt_*' | wc -l)
ext=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_extended*' | wc -l)
echo "tt_total=$total tt_pending=$pending tt_active=$((total-pending)) tt_extended=$ext"
echo "sumo=$SUMO"

ok=0; bad=0
for d in tools/oracle/cases/tt_extended*/; do
  [ -f "$d/routes.rou" ] || continue
  [ -f "$d/ref.tripinfo.xml" ] || continue
  c=$(basename "$d")
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout 90 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  if [ "$rc" -ne 0 ]; then
    printf 'SIMFAIL %-60s rc=%s %s\n' "$c" "$rc" "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-140)"
    bad=$((bad+1)); continue
  fi
  r="PASS"; dd=""
  if ! python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" >"$OUT/ti.cmp" 2>&1; then
    r="FAIL"; dd="ti:$(grep -m1 -E 'DIFF|UNREADABLE|SUMMARY: [1-9]' "$OUT/ti.cmp" | cut -c1-160)"
  fi
  if [ -f "$d/ref.fcd.xml" ] && ! python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" >"$OUT/fcd.cmp" 2>&1; then
    r="FAIL"; dd="$dd fcd:$(grep -m1 -E 'DIFF|UNREADABLE|SUMMARY: [1-9]' "$OUT/fcd.cmp" | cut -c1-160)"
  fi
  if [ -f "$d/ref.stat.xml" ] && ! python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" >"$OUT/stat.cmp" 2>&1; then
    r="FAIL"; dd="$dd stat:$(grep -m1 -E 'DIFF|UNREADABLE|SUMMARY: [1-9]' "$OUT/stat.cmp" | cut -c1-160)"
  fi
  if [ "$r" = PASS ]; then ok=$((ok+1)); else printf 'DIFF %-60s %s\n' "$c" "$(echo "$dd" | cut -c1-200)"; bad=$((bad+1)); fi
done
echo "EXTENDED ok=$ok bad=$bad"
