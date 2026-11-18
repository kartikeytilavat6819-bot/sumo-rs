#!/bin/bash
# Diagnostic for oc67: run imported netconvert rows and dump failing diffs.
OUT=/tmp/oc67_nc
rm -rf "$OUT"
mapfile -t ROWS < <(awk '{print $2}' tools/oracle/import_texttest_net.rows)
if [ -n "$OC67_FILTER" ]; then
  F=(); for r in "${ROWS[@]}"; do [[ "$r" == *"$OC67_FILTER"* ]] && F+=("$r"); done; ROWS=("${F[@]}")
fi
if [ -n "$OC67_LIMIT" ]; then ROWS=("${ROWS[@]:0:$OC67_LIMIT}"); fi
echo "running ${#ROWS[@]} imported rows"
NETOPTS2_OUT="$OUT" bash tools/oracle/run_netopts_new.sh "${ROWS[@]}" 2>&1 \
  | grep -E 'FAIL|PANIC|TOTAL' | head -200
echo "=== diffs ==="
awk -F'\t' '$2!="PASS"{print $1}' "$OUT/summary.tsv" | while read -r v; do
  echo "--- $v ---"
  head -20 "$OUT/$v/net.log" 2>/dev/null
  for f in $(cd "$OUT/$v/cpp_out" 2>/dev/null && find . -type f); do
    if ! cmp -s "$OUT/$v/cpp_out/$f" "$OUT/$v/rust_out/$f" 2>/dev/null; then
      echo "FILE $f"
      diff <(sed -e 's/\r$//; /generated on/d; s/date="[^"]*"/date="TIME"/g' "$OUT/$v/cpp_out/$f") \
           <(sed -e 's/\r$//; /generated on/d; s/date="[^"]*"/date="TIME"/g' "$OUT/$v/rust_out/$f") | head -40
    fi
  done
done
