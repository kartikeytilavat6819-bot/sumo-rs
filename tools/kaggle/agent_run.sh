#!/bin/bash
# oc50: measure the TextTest-imported oracle rows on the gate VM.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc50_out
rm -rf "$OUT"; mkdir -p "$OUT"

# the Kaggle gate only builds sumo + netconvert; duarouter cases need their binary
export PATH=$HOME/.cargo/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
if [ ! -x "$CARGO_TARGET_DIR/debug/duarouter" ]; then
  timeout 300 cargo build -p duarouter --message-format short >"$OUT/build.log" 2>&1 || tail -10 "$OUT/build.log"
fi

mapfile -t NC < <(sed -n 's/^row \([^ ]*\) .*/\1/p' tools/oracle/import_texttest_net.rows)
echo "IMPORTED NC ROWS: ${#NC[@]}"
NETOPTS2_JOBS=4 NETOPTS2_OUT="$OUT/nc" timeout 1200 \
  bash tools/oracle/run_netopts_new.sh "${NC[@]}" >"$OUT/nc.log" 2>&1
echo "--- netconvert non-PASS (name verdict detail) ---"
grep -v $'\tPASS\t' "$OUT/nc/summary.tsv" 2>/dev/null | head -50
echo "--- netconvert totals ---"
awk -F'\t' '{c[$2]++} END {for (k in c) print k, c[k]}' "$OUT/nc/summary.tsv" 2>/dev/null

mapfile -t DU < <(cd tools/oracle/cases && ls -d duarouter_itt_* 2>/dev/null)
echo "IMPORTED DUA CASES: ${#DU[@]}"
OUT="$OUT/dua" timeout 600 bash tools/oracle/run_duarouter.sh "${DU[@]}" >"$OUT/dua.log" 2>&1
echo "--- duarouter non-PASS ---"
grep -v $'\tPASS\t' "$OUT/dua/results.tsv" 2>/dev/null | tail -n +2 | head -30
echo "--- duarouter totals ---"
awk -F'\t' 'NR>1 {c[$2]++} END {for (k in c) print k, c[k]}' "$OUT/dua/results.tsv" 2>/dev/null
