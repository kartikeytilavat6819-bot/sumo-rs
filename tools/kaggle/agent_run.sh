#!/bin/bash
# oc63: verify geometry.remove fix on the failing seeds + fuzz a fresh range.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
if [ ! -x "$CARGO_TARGET_DIR/debug/netgenerate" ]; then
  echo "building netgenerate..."
  cargo build -p netgenerate --message-format short 2>&1 | grep -E ': error|^error' | head -20
fi
echo "ours ng: $(test -x "$CARGO_TARGET_DIR/debug/netgenerate" && echo yes || echo no); official ng: $(test -x "$SUMO_PRISTINE_BIN/netgenerate" && echo yes || echo no)"
export FUZZ_OUT=/tmp/fuzz_net_oc63_round2
mkdir -p "$FUZZ_OUT"
export FUZZ_FIRST=130
export FUZZ_BUDGET=1400
bash tools/oracle/fuzz_net.sh 500 > "$FUZZ_OUT/all.log" 2>&1
echo "=== SUMMARY ==="
grep '^=== PASS' "$FUZZ_OUT/all.log"
echo "=== FAIL/PANIC status lines (first 60) ==="
grep -E ' (FAIL|PANIC) ' "$FUZZ_OUT/all.log" | head -60
echo "=== FAILED details block (first 200 lines) ==="
sed -n '/=== FAILED details/,$p' "$FUZZ_OUT/all.log" | head -200
