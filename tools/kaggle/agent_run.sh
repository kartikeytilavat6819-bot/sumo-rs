#!/bin/bash
# Gate diagnostic for oc57: scan imported tt_bugs_* rows, print per-case status and first diff.
# The gate already runs the full scoreboard; this only adds focused detail for my subtrees.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc57-scan
rm -rf "$OUT"; mkdir -p "$OUT" || exit 1
n=0; ok=0; bad=0
for d in tools/oracle/cases/tt_bugs_*/; do
  c=$(basename "$d")
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args" 2>/dev/null)
  n=$((n+1))
  timeout 30 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then
    printf '%-58s PANIC rc=%s %s\n' "$c" "$rc" "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-100)"
    bad=$((bad+1)); continue
  fi
  detail=""
  e=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
  [ -n "$e" ] && detail="TI:$e"
  if [ -z "$detail" ] && [ -f "$d/ref.fcd.xml" ]; then
    e=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
    [ -n "$e" ] && detail="FCD:$e"
  fi
  if [ -z "$detail" ] && [ -f "$d/ref.stat.xml" ]; then
    e=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" 2>&1 | head -1)
    [ -n "$e" ] && detail="STAT:$e"
  fi
  if [ -z "$detail" ]; then
    ok=$((ok+1))
  else
    bad=$((bad+1))
    printf '%-58s FAIL %s\n' "$c" "$(echo "$detail" | cut -c1-220)"
  fi
done
echo "oc57-scan cases=$n pass=$ok fail=$bad"
