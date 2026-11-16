#!/bin/bash
# Scoreboard: run every oracle case through our binaries and print PASS/FAIL per stage.
# Usage: bash tools/oracle/run_all.sh [--build]
set -u
cd /c/Users/karti/sumo-rs || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-main
export CARGO_BUILD_JOBS=8
BIN="$CARGO_TARGET_DIR/debug"
if [ "${1:-}" = "--build" ]; then
  cargo build -q -p sumo -p netconvert 2>&1 | grep -E '^error' | head -5
fi
OUT=/c/Users/karti/Temp/score
mkdir -p "$OUT"
PASS=0; FAIL=0; SKIP=0

line() { printf '%-22s %-10s %s\n' "$1" "$2" "$3"; }

for dir in tools/oracle/cases/*/; do
  c=$(basename "$dir")
  [ -f "$dir/input.osm" ] || continue
  # --- net stage
  if [ -f "$dir/ref_simple.net.xml" ]; then
    "$BIN/netconvert.exe" --osm-files "$dir/input.osm" --simple-projection \
        -o "$OUT/$c.net.xml" >/dev/null 2>&1
    if python tools/oracle/compare_net.py "$dir/ref_simple.net.xml" "$OUT/$c.net.xml" >"$OUT/$c.net.log" 2>&1; then
      line "$c" net PASS; PASS=$((PASS+1))
    else
      line "$c" net FAIL "$(grep SUMMARY "$OUT/$c.net.log" | head -1)"; FAIL=$((FAIL+1))
    fi
  else
    line "$c" net SKIP "no reference net"; SKIP=$((SKIP+1))
  fi
  # --- sim stage
  if [ -f "$dir/routes.rou" ] && [ -f "$dir/ref_simple.net.xml" ] && [ -f "$dir/ref.tripinfo.xml" ]; then
    EXTRA=""
    [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
    "$BIN/sumo.exe" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
        --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA \
        >"$OUT/$c.sumo.log" 2>&1
    rc=$?
    if [ $rc -ne 0 ]; then
      line "$c" sim-PANIC "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.sumo.log" | cut -c1-90)"; FAIL=$((FAIL+1))
      continue
    fi
    r="PASS"
    python tools/oracle/compare_sim.py --tripinfo "$dir/ref.tripinfo.xml" "$OUT/$c.ti.xml" >"$OUT/$c.ti.log" 2>&1 || r="FAIL"
    if [ -f "$dir/ref.fcd.xml" ]; then
      python tools/oracle/compare_sim.py --fcd "$dir/ref.fcd.xml" "$OUT/$c.fcd.xml" >"$OUT/$c.fcd.log" 2>&1 || r="FAIL"
    fi
    if [ -f "$dir/ref.stat.xml" ]; then
      python tools/oracle/compare_stat.py "$dir/ref.stat.xml" "$OUT/$c.stat.xml" >"$OUT/$c.stat.log" 2>&1 || r="FAIL"
    fi
    if [ "$r" = "PASS" ]; then
      line "$c" sim PASS; PASS=$((PASS+1))
    else
      d=$( (head -1 "$OUT/$c.ti.log"; head -1 "$OUT/$c.fcd.log"; tail -1 "$OUT/$c.stat.log") 2>/dev/null | grep -vE '^$' | head -1 | cut -c1-90)
      line "$c" sim FAIL "$d"; FAIL=$((FAIL+1))
    fi
  else
    line "$c" sim SKIP "no routes/refs"; SKIP=$((SKIP+1))
  fi
done
echo "-------------------------------------------"
echo "PASS=$PASS FAIL=$FAIL SKIP=$SKIP"
