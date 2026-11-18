#!/bin/bash
# oc70: measure the duarouter person/trips/trips_geo/personFlow/container TextTest import.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc70_out
rm -rf "$OUT"; mkdir -p "$OUT"

# the Kaggle gate only builds sumo + netconvert; duarouter cases need their binary
export PATH=$HOME/.cargo/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
if [ ! -x "$CARGO_TARGET_DIR/debug/duarouter" ]; then
  timeout 300 cargo build -p duarouter --message-format short >"$OUT/build.log" 2>&1 || tail -10 "$OUT/build.log"
fi

export RUST_BACKTRACE=1
mapfile -t DU < <(cd tools/oracle/cases && ls -d duarouter_itt_* 2>/dev/null)
echo "IMPORTED DUA CASES: ${#DU[@]}"
OUT="$OUT/dua" timeout 900 bash tools/oracle/run_duarouter.sh "${DU[@]}" >"$OUT/dua.log" 2>&1
R="$OUT/dua/results.tsv"
echo "--- duarouter totals ---"
awk -F'\t' 'NR>1 {c[$2]++} END {for (k in c) print k, c[k]}' "$R" 2>/dev/null
echo "--- duarouter non-PASS ---"
grep -v $'\tPASS\t' "$R" 2>/dev/null | tail -n +2 | head -160
echo "--- first failing-case diffs (<=3) ---"
n=0
while IFS=$'\t' read -r case verdict rrc crc detail; do
  [ "$verdict" = "PASS" ] && continue
  [ "$n" -ge 3 ] && break
  echo "=== $case $verdict rc=$rrc/$crc :: $detail ==="
  for f in "$OUT/dua/$case"/*.diff; do
    [ -f "$f" ] || continue
    echo "--- $(basename "$f") ---"
    head -30 "$f"
  done
  echo "--- cand.log tail ---"
  if [ "$verdict" = "PANIC" ]; then
    grep -A40 'panicked at' "$OUT/dua/$case/cand.log" 2>/dev/null | head -45
  else
    head -8 "$OUT/dua/$case/cand.log" 2>/dev/null
  fi
  n=$((n+1))
done < "$R"
echo "--- rail + taxi diffs ---"
for c in duarouter_itt_duarouter_trips_rail_reversal_cost duarouter_itt_duarouter_person_personTrip_taxi_walk_last duarouter_itt_duarouter_trips_bulk_routing_bulk_reset_after_trivial_trip; do
  echo "=== $c ==="
  cat "$OUT/dua/$c"/*.diff 2>/dev/null | head -30
done
echo "--- PANIC frames (first 3) ---"
n=0
grep -l 'panicked at' "$OUT/dua"/*/cand.log 2>/dev/null | while read -r f; do
  [ "$n" -ge 3 ] && break
  echo "=== $(basename "$(dirname "$f")") ==="
  grep -A12 'panicked at' "$f" | head -14
  n=$((n+1))
done
