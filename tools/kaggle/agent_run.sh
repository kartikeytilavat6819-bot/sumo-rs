#!/bin/bash
# oc45 probe: differential fuzz of netgenerate/netconvert vs official SUMO 1.28.0.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
# The gate may invoke this from a directory other than the source tree; find the tree by the fuzzer itself.
HERE=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
ROOT=""
for c in "$HERE/../.." /tmp/src /kaggle/working/src /home/gate/src "$(pwd)"; do
  [ -f "$c/tools/oracle/fuzz_net.sh" ] && { ROOT=$(cd "$c" && pwd); break; }
done
if [ -z "$ROOT" ]; then
  f=$(find /tmp /home /kaggle /mnt -path '*/tools/oracle/fuzz_net.sh' -print -quit 2>/dev/null)
  [ -n "$f" ] && ROOT=${f%/tools/oracle/fuzz_net.sh}
fi
if [ -z "$ROOT" ]; then echo "PROBE: cannot locate tools/oracle/fuzz_net.sh"; exit 1; fi
cd "$ROOT" || exit 1
echo "PROBE root=$ROOT"
# gate builds sumo+netconvert only; netgenerate must be built here
if [ ! -x "$CARGO_TARGET_DIR/debug/netgenerate" ]; then
  echo "building netgenerate..."
  cargo build -p netgenerate --message-format short 2>&1 | grep -E ': error|^error' | head -20
fi
echo "ours ng: $(test -x "$CARGO_TARGET_DIR/debug/netgenerate" && echo yes || echo no); official ng: $(test -x "$SUMO_PRISTINE_BIN/netgenerate" && echo yes || echo no)"
FUZZ_BUDGET=450 bash "$ROOT/tools/oracle/fuzz_net.sh" 20 2>&1 | head -180
