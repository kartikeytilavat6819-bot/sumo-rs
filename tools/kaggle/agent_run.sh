#!/bin/bash
# Gate diagnostic for oc49: run the oracle scoreboard, report imported TextTest
# rows and the first few failure diffs.  Invoked by the gate after the tests.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
export SCORE_OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc49-score
rm -rf "$SCORE_OUT"; mkdir -p "$SCORE_OUT"

bash tools/oracle/run_all.sh > "$SCORE_OUT/all.txt" 2>&1

echo "=== imported TextTest rows (first 120) ==="
grep -E '^tt_' "$SCORE_OUT/all.txt" | head -120
echo "=== summary ==="
grep -E '^(PASS=|STALE|MISSING)' "$SCORE_OUT/all.txt" | tail -5

echo "=== first 8 failing imported rows with diff ==="
n=0
while read -r row; do
  c=$(echo "$row" | awk '{print $1}')
  [ -z "$c" ] && continue
  # strip a /ref suffix used by variant rows
  base=${c%%/*}
  echo "--- $row"
  for f in "$SCORE_OUT/$c.ti.log" "$SCORE_OUT/$base.ref.ti.log" "$SCORE_OUT/$c.fcd.log" "$SCORE_OUT/$c.stat.log" "$SCORE_OUT/$base.ref.stat.log"; do
    [ -f "$f" ] && { echo "[$(basename "$f")]"; head -4 "$f"; }
  done
  n=$((n+1)); [ "$n" -ge 8 ] && break
done < <(grep -E '^tt_' "$SCORE_OUT/all.txt" | grep -E 'FAIL|PANIC|TIMEOUT' | head -8)
