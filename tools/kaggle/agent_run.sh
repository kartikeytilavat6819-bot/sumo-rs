#!/bin/bash
# oc38 diagnostic: compare activitygen behaviour on a bad cityGate edge (the
# parseCityGates try/catch path) against the official binary.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/activitygen
REF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/activitygen
NET=tools/oracle/cases/activitygen_data/activitygen-example.net.xml
STAT=tools/oracle/cases/activitygen_data/activitygen-example.stat.xml
D=$(mktemp -d)
sed 's/edge="e01t11" pos="1"/edge="NO_SUCH_EDGE" pos="1"/' "$STAT" > "$D/bad.stat.xml"
for side in ref cand; do
  if [ "$side" = ref ]; then B=$REF; else B=$BIN; fi
  "$B" --net-file "$NET" --stat-file "$D/bad.stat.xml" \
      --output-file "$D/$side.rou.xml" --seed 1 \
      >"$D/$side.out" 2>"$D/$side.err"
  echo "$side rc=$?"
done
echo "=== ref stdout ==="; cat "$D/ref.out"
echo "=== cand stdout ==="; cat "$D/cand.out"
echo "=== ref stderr ==="; cat "$D/ref.err"
echo "=== cand stderr ==="; cat "$D/cand.err"
echo "=== stdout diff ==="; diff "$D/ref.out" "$D/cand.out" | head -30
echo "=== stderr diff ==="; diff "$D/ref.err" "$D/cand.err" | head -30
