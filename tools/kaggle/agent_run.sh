#!/bin/bash
# Gate diagnostic for oc56: run imported rail cases and print the full first panic/diff line.
# CASE=<name> narrows to one case. Uses the same flags as run_all.sh.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT="${TMPDIR:-/tmp}/oc56-rail"
rm -rf "$OUT"; mkdir -p "$OUT"
CASE="${CASE:-}"

cases="$CASE"
if [ -z "$CASE" ]; then
  cases=$(ls -d tools/oracle/cases/tt_rail_* 2>/dev/null | xargs -n1 basename | tr '\n' ' ')
fi
pass=0; fail=0
for c in $cases; do
  d=tools/oracle/cases/$c
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args")
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  r="PASS"
  diffline=""
  if [ $rc -ne 0 ]; then
    r="PANIC"
    diffline=$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-200)
  else
    d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
    [ -n "$d1" ] && { r="FAIL"; diffline="$d1"; }
    if [ -f "$d/ref.fcd.xml" ]; then
      d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
      [ -n "$d2" ] && { r="FAIL"; diffline="$diffline | fcd: $d2"; }
    fi
    if [ -f "$d/ref.stat.xml" ]; then
      d3=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" 2>&1 | head -1)
      [ -n "$d3" ] && { r="FAIL"; diffline="$diffline | stat: $d3"; }
    fi
  fi
  printf '%-64s %s %s\n' "$c" "$r" "$diffline"
  [ "$r" = PASS ] && pass=$((pass+1)) || fail=$((fail+1))
done
echo "-----------------------------------------------------------"
echo "rail pass=$pass fail=$fail"
