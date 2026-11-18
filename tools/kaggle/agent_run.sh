#!/bin/bash
# oc63: fuzz a fresh seed range after the geometry.remove fix.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
export FUZZ_OUT=/tmp/fuzz_net_oc63_round3
mkdir -p "$FUZZ_OUT"
export FUZZ_FIRST=1001
export FUZZ_BUDGET=1400
bash tools/oracle/fuzz_net.sh 500 > "$FUZZ_OUT/all.log" 2>&1
echo "=== SUMMARY ==="
grep '^=== PASS' "$FUZZ_OUT/all.log"
echo "=== FAIL/PANIC status lines (first 60) ==="
grep -E ' (FAIL|PANIC) ' "$FUZZ_OUT/all.log" | head -60
echo "=== FAILED details block (first 200 lines) ==="
sed -n '/=== FAILED details/,$p' "$FUZZ_OUT/all.log" | head -200
