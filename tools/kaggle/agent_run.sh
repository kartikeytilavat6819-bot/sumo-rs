#!/bin/bash
# oc45 probe: differential fuzz of netgenerate/netconvert vs official SUMO 1.28.0.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
cd "$(dirname "$0")/../.." || exit 1
# gate builds sumo+netconvert only; netgenerate must be built here
if [ ! -x "$CARGO_TARGET_DIR/debug/netgenerate" ]; then
  echo "building netgenerate..."
  cargo build -p netgenerate --message-format short 2>&1 | grep -E ': error|^error' | head -20
fi
echo "ours ng: $(test -x "$CARGO_TARGET_DIR/debug/netgenerate" && echo yes || echo no); official ng: $(test -x "$SUMO_PRISTINE_BIN/netgenerate" && echo yes || echo no)"
FUZZ_BUDGET=450 bash tools/oracle/fuzz_net.sh 20 2>&1 | head -180
