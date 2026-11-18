#!/bin/bash
# oc44 differential fuzz measurement (branch-only; removed before final commit).
out=/tmp/oc44_fuzz.txt
FUZZ_FIRST=${FUZZ_FIRST:-7000} FUZZ_BUDGET=${FUZZ_BUDGET:-400} bash tools/oracle/fuzz_sim.sh 120 > "$out" 2>&1
echo "=== NON-PASS LINES (shrunk cases included) ==="
grep -v '=> PASS' "$out" | head -390
echo "=== TOTAL LINES: $(wc -l < "$out") ==="
