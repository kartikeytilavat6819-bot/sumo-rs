#!/bin/bash
# oc71: measure imported duarouter TextTest rows on the gate VM.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc71_out
rm -rf "$OUT"; mkdir -p "$OUT"

# the Kaggle gate only builds sumo + netconvert; duarouter cases need their binary
export PATH=$HOME/.cargo/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
if [ ! -x "$CARGO_TARGET_DIR/debug/duarouter" ]; then
  timeout 400 cargo build -p duarouter --message-format short >"$OUT/build.log" 2>&1 || tail -10 "$OUT/build.log"
fi

mapfile -t DU < <(cd tools/oracle/cases && ls -d duarouter_itt_* 2>/dev/null)
echo "DUA CASES: ${#DU[@]}"
OUT="$OUT/dua" timeout 900 bash tools/oracle/run_duarouter.sh "${DU[@]}" >"$OUT/dua.log" 2>&1
echo "--- verdict totals ---"
awk -F'\t' 'NR>1 {c[$2]++} END {for (k in c) print k, c[k]}' "$OUT/dua/results.tsv" 2>/dev/null
echo "--- non-PASS (case verdict ref_rc cand_rc detail) ---"
grep -v $'\tPASS\t' "$OUT/dua/results.tsv" 2>/dev/null | tail -n +2 | head -40
echo "--- first diffs of FAIL rows ---"
n=0
while IFS=$'\t' read -r c v rrc crc detail; do
  [ "$v" = FAIL ] || continue
  n=$((n+1)); [ $n -gt 6 ] && break
  echo "=== $c : $detail ==="
  head -24 "$OUT/dua/$c"/*.diff 2>/dev/null
done < "$OUT/dua/results.tsv"
