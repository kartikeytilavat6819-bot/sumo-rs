#!/bin/bash
# Runs our sumo on every depart variant (one route file each) against the C++ reference in ref/<variant>/
# (made by run_ref.sh, junction net, --seed 42 --end 120). Not part of run_all.sh: 15 variants, 15 refs.
# Verdict per variant:
#   reference wrote an `Error:` line -> PASS only if ours exits non-zero with the same Error line;
#   otherwise                       -> PASS only if ours exits 0 and tripinfo + statistics match.
# Outputs go to $OUT, not the git-tracked ours/ directory.
# Usage: bash tools/oracle/cases/depart/run_all_variants.sh   (CARGO_TARGET_DIR picks the build under test)
set -u
D=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$D/../../../.." && pwd)
BIN=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-main}/debug/sumo.exe
OUT=/c/Users/karti/Temp/score/depart
mkdir -p "$OUT"
cd "$ROOT" || exit 1
PASS=0; FAIL=0
for r in "$D"/*.rou; do
  n=$(basename "$r" .rou)
  o="$OUT/$n"; mkdir -p "$o"; rm -f "$o"/*
  ref="$D/ref/$n"
  "$BIN" -n tools/oracle/cases/junction/ref_simple.net.xml -r "$r" \
    --tripinfo-output "$o/tripinfo.xml" --statistic-output "$o/statistic.xml" \
    --seed 42 --end 120 --no-step-log >"$o/stdout.txt" 2>"$o/stderr.txt"
  rc=$?
  if grep -q '^Error:' "$ref/stderr.txt"; then
    # shortcut: CR stripped because the committed ref stderr has CRLF (GNU grep keeps it, MSYS grep drops it), upgrade when refs are LF
    if [ $rc -ne 0 ] && [ "$(grep -m1 '^Error:' "$o/stderr.txt" | tr -d '\r')" = "$(grep -m1 '^Error:' "$ref/stderr.txt" | tr -d '\r')" ]; then
      v=PASS; d="error parity"
    else
      v=FAIL; d="ref: $(grep -m1 '^Error:' "$ref/stderr.txt" | cut -c1-70) / ours rc=$rc"
    fi
  elif [ $rc -ne 0 ]; then
    v=FAIL; d="ours rc=$rc: $(grep -m1 -E 'Error|panicked' "$o/stderr.txt" | cut -c1-70)"
  else
    if python tools/oracle/compare_sim.py --tripinfo "$ref/tripinfo.xml" "$o/tripinfo.xml" >"$o/ti.log" 2>&1 \
       && python tools/oracle/compare_stat.py "$ref/statistic.xml" "$o/statistic.xml" >"$o/stat.log" 2>&1; then
      v=PASS; d=""
    else
      v=FAIL; d=$( (head -1 "$o/ti.log"; tail -1 "$o/stat.log") 2>/dev/null | grep -v '^$' | head -1 | cut -c1-90)
    fi
  fi
  [ "$v" = PASS ] && PASS=$((PASS+1)) || FAIL=$((FAIL+1))
  printf '%-16s %-5s %s\n' "$n" "$v" "$d"
done
echo "-------------------------------------------"
echo "depart variants: PASS=$PASS FAIL=$FAIL"
