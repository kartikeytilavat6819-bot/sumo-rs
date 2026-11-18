#!/bin/bash
# Gate diagnostic for oc53: run our sumo on each active tt_output row and show first diff line.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"; [ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc53
rm -rf "$OUT"; mkdir -p "$OUT"
pass=0; fail=0; shown=0
for d in tools/oracle/cases/tt_output_*/; do
  c=$(basename "$d")
  [ -f "$d/PENDING" ] && continue
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args")
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true $flags \
      >"$OUT/$c.log" 2>&1
  rc=$?
  res="PASS"
  [ $rc -ne 0 ] && res="PANIC(rc=$rc)"
  if [ $rc -eq 0 ]; then
    x=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -1)
    [ -n "$x" ] && res="TI:$x"
    [ "$res" = PASS ] && [ -f "$d/ref.fcd.xml" ] && {
      x=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | head -1)
      [ -n "$x" ] && res="FCD:$x"; }
    [ "$res" = PASS ] && [ -f "$d/ref.stat.xml" ] && {
      x=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/$c.stat.xml" 2>&1 | head -1)
      case "$x" in *"0 differences"*) ;; [ -z "$x" ] ;; *) res="STAT:$x";; esac; }
  fi
  if [ "$res" = PASS ]; then pass=$((pass+1)); else
    fail=$((fail+1))
    if [ $shown -lt 25 ]; then
      printf 'FAIL %-58s %s\n' "$c" "$res"
      shown=$((shown+1))
    fi
  fi
done
echo "oc53 SUMMARY pass=$pass fail=$fail"
