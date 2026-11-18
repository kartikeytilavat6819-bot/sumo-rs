#!/bin/bash
# Gate diagnostic for oc53: run our sumo on tt_output rows and show failures / panic lines.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
SUMO="$BIN/sumo.exe"; [ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc53
rm -rf "$OUT"; mkdir -p "$OUT" || exit 1
pass=0; fail=0; shown=0
for d in tools/oracle/cases/tt_output_*/; do
  c=$(basename "$d")
  [ -f "$d/PENDING" ] && continue
  [ -f "$d/ref.tripinfo.xml" ] || continue
  if [ $# -gt 0 ]; then
    want=0; for a in "$@"; do case "$c" in *"$a"*) want=1;; esac; done
    [ $want -eq 1 ] || continue
  fi
  flags=$(cat "$d/sim.args")
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true $flags \
      >"$OUT/$c.log" 2>&1
  rc=$?
  res="PASS"
  if [ $rc -ne 0 ]; then
    res="PANIC: $(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-160)"
    pline=$(grep -m1 -A1 'panicked at' "$OUT/$c.log" | tail -1 | cut -c1-160)
    [ -n "$pline" ] && res="$res || $pline"
  else
    x=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -1)
    case "$x" in ""|MATCH) ;; *) res="TI:$x";; esac
    if [ "$res" = PASS ] && [ -f "$d/ref.fcd.xml" ]; then
      x=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | head -1)
      case "$x" in ""|MATCH) ;; *) res="FCD:$x";; esac
    fi
    if [ "$res" = PASS ] && [ -f "$d/ref.stat.xml" ]; then
      x=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/$c.stat.xml" 2>&1 | head -1)
      case "$x" in *"0 differences"*|"") ;; *) res="STAT:$x";; esac
    fi
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
