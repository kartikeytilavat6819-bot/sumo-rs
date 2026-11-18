#!/bin/bash
# oc38 diagnostics: (1) activitygen bad cityGate error path vs official;
# (2) the netopts_new opendriveOutput row flagged as a regression.
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
echo "=== activitygen stderr diff (ref vs cand) ==="
diff "$D/ref.err" "$D/cand.err" | head -30

echo "=== netopts opendriveOutput ==="
mkdir -p "$D/netopts"
NETOPTS2_OUT="$D/netopts" bash tools/oracle/run_netopts_new.sh opendriveOutput 2>&1 | tail -20
echo "--- summary ---"
cat "$D/netopts/summary.tsv" 2>/dev/null | head -10
echo "--- net.log ---"
head -40 "$D/netopts/opendriveOutput/net.log" 2>/dev/null
echo "--- xodr diff (first 25) ---"
diff "$D/netopts/opendriveOutput/cpp_out/net.xodr" "$D/netopts/opendriveOutput/rust_out/net.xodr" 2>/dev/null | head -25
