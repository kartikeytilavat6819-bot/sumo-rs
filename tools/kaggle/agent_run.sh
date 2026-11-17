#!/bin/bash
# ou03 measurement: output-file oracle on our build vs committed C++ references (REMOVE before final commit).
set -u
export BIN=$CARGO_TARGET_DIR/debug/sumo
export OUT=/tmp/ou03_outputs
export REFS=committed
bash tools/oracle/run_outputs.sh
for f in $OUT/*/*/compare.log; do
  [ -f "$f" ] || continue
  echo "== $f"
  head -4 "$f" | cut -c1-220
done
