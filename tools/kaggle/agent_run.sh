#!/bin/bash
# Diagnostic for oc67: run the imported netconvert TextTest rows against the pristine binary
# and print only failures, plus a sample of the divergence details.
OUT=/tmp/oc67_nc
rm -rf "$OUT"
mapfile -t ROWS < <(awk '{print $2}' tools/oracle/import_texttest_net.rows)
if [ -n "$OC67_LIMIT" ]; then ROWS=("${ROWS[@]:0:$OC67_LIMIT}"); fi
echo "running ${#ROWS[@]} imported rows"
NETOPTS2_OUT="$OUT" bash tools/oracle/run_netopts_new.sh "${ROWS[@]}" 2>&1 \
  | grep -E 'FAIL|PANIC|TOTAL' | head -200
echo "=== first divergence details ==="
awk -F'\t' '$2!="PASS" && $1 ~ /^itt_nc_/ {print $1": "$3}' "$OUT/summary.tsv" | head -40
