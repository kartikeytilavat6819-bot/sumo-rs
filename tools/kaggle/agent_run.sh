#!/bin/bash
# oc29 arc-flag diagnostics. Runs on the gate after tests.
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}
BIN="$CARGO_TARGET_DIR/debug/duarouter"
REF="$SUMO_PRISTINE_BIN/duarouter"
echo "=== BIN=$BIN REF=$REF ==="

echo "=== [1] duarouter oracle: arcflag case (verbose) ==="
OUT=/tmp/arcflag_oracle
rm -rf "$OUT"
OUT="$OUT" bash tools/oracle/run_duarouter.sh duarouter_arcflag 2>&1 | tail -5
echo "--- ref log ---"; sed -n '1,40p' "$OUT/duarouter_arcflag/ref.log" 2>/dev/null
echo "--- cand log ---"; sed -n '1,40p' "$OUT/duarouter_arcflag/cand.log" 2>/dev/null
echo "--- routes diff (head) ---"; head -40 "$OUT/duarouter_arcflag/routes.rou.xml.diff" 2>/dev/null

echo "=== [2] all duarouter cases (results.tsv) ==="
OUT=/tmp/duarouter_all
rm -rf "$OUT"
OUT="$OUT" bash tools/oracle/run_duarouter.sh 2>&1 | tail -5
cat "$OUT/results.tsv" 2>/dev/null

echo "=== [3] direct arcflag via official (head of output) ==="
mkdir -p /tmp/arcflag_direct
"$REF" -n tools/oracle/cases/trips/ref_simple.net.xml \
  --trip-files tools/oracle/cases/duarouter_trips/trips.xml \
  --routing-algorithm arcflag --output-file /tmp/arcflag_direct/routes.rou.xml \
  --no-step-log true >/tmp/arcflag_direct/ref.log 2>&1
echo "ref rc=$?"; sed -n '1,30p' /tmp/arcflag_direct/ref.log
head -20 /tmp/arcflag_direct/routes.rou.xml 2>/dev/null
