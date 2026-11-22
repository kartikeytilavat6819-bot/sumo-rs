#!/bin/bash
# Probe the 4 complex regressions: official vs ours, full first diff per test.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc100_regress
rm -rf "$OUT"
mkdir -p "$OUT"
for c in misc/concurrent person/person simulation/loadState/persons_include simulation/simulation; do
  key=$(echo "$c" | tr / _)
  for side in official ours; do
    d="$OUT/$key/$side"
    cp -r "tools/oracle/complex_bundle/$c" "$d"
    if [ "$side" = official ]; then BIN="$SUMO_PRISTINE_BIN/sumo"; else BIN="$CARGO_TARGET_DIR/debug/sumo"; fi
    (cd "$d" && SUMO_BINARY="$BIN" PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}" python3 runner.py > console.txt 2>&1)
  done
  echo "===== $c"
  diff -u "$OUT/$key/official/console.txt" "$OUT/$key/ours/console.txt" | head -80
  if [ -f "$OUT/$key/official/state.xml" ] || [ -f "$OUT/$key/ours/state.xml" ]; then
    echo "----- state.xml"
    diff -u "$OUT/$key/official/state.xml" "$OUT/$key/ours/state.xml" | head -40
  fi
done
