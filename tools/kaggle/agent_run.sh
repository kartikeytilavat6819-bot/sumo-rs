#!/bin/bash
# oc31 measurement: A* landmark lookup tables (edge ids + geo positions)
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
cd "$ROOT" || exit 1
export OUT=/tmp/oc31_duar
mkdir -p "$OUT"
bash tools/oracle/run_duarouter.sh duarouter_alt_landmark duarouter_astar_landmark_ids duarouter_astar_landmark_geo 2>&1 | tail -20
echo "=== results ==="
cat "$OUT/results.tsv" 2>/dev/null
echo "=== cand logs (panics/errors) ==="
for c in duarouter_alt_landmark duarouter_astar_landmark_ids duarouter_astar_landmark_geo; do
  echo "--- $c"
  grep -iE 'panic|error|warning|landmark|no edge|coordinate' "$OUT/$c/cand.log" 2>/dev/null | head -12
done
echo "=== saved landmark / routes diffs (head) ==="
for c in duarouter_alt_landmark duarouter_astar_landmark_ids duarouter_astar_landmark_geo; do
  echo "--- $c"
  head -8 "$OUT/$c/saved_landmarks.txt.diff" 2>/dev/null
  head -8 "$OUT/$c/routes.rou.xml.diff" 2>/dev/null
done
