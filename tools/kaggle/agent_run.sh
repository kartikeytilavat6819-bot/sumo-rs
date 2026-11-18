#!/bin/bash
# oc44 differential fuzz measurement (branch-only; removed before final commit).
for first in 14000 15000; do
  echo "=== RANGE $first ==="
  out=/tmp/oc44_fuzz_$first.txt
  FUZZ_FIRST=$first FUZZ_BUDGET=280 bash tools/oracle/fuzz_sim.sh 100 > "$out" 2>&1
  grep -v '=> PASS' "$out" | head -190
  echo "=== RANGE $first TOTAL LINES: $(wc -l < "$out") ==="
done
