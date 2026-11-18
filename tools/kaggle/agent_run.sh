#!/bin/bash
# oc25 measurement: run the marouter oracle, list results and any failing diff.
set -u
cd /tmp/src 2>/dev/null || cd "$(dirname "$0")/../.." || exit 1
export PATH=$HOME/.cargo/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export SUMO_HOME=${SUMO_HOME:-/tmp/sumo_pkg/sumo}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
OUT=/tmp/oc25_marouter
rm -rf "$OUT"
timeout 600 bash tools/oracle/run_odtool.sh marouter >"$OUT.run.log" 2>&1
echo "=== run_odtool.sh tail ==="
tail -20 "$OUT.run.log"
echo "=== results.tsv ==="
cat "$OUT/results.tsv" 2>/dev/null
echo "=== failing details ==="
awk -F'\t' 'NR>1 && $2!="PASS"{print $1}' "$OUT/results.tsv" 2>/dev/null | while read -r c; do
  echo "--- case $c ---"
  head -20 "$OUT/$c/cand.log" 2>/dev/null
  for f in "$OUT/$c/ref"/*; do
    n=$(basename "$f")
    [ -f "$OUT/$c/cand/$n" ] || continue
    echo "### diff $n"
    diff <(grep -v 'generated on' "$f" | sed -E 's#/(ref|cand)/#/SIDE/#g') \
         <(grep -v 'generated on' "$OUT/$c/cand/$n" | sed -E 's#/(ref|cand)/#/SIDE/#g') | head -40
  done
done
