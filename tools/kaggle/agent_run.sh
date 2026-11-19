#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
CASES="duarouter_itt_duarouter_filter_files duarouter_itt_duarouter_personFlow_edges duarouter_itt_duarouter_personFlow_walk_poisson duarouter_itt_duarouter_person_edges duarouter_itt_duarouter_person_routeID duarouter_itt_duarouter_person_unconnected_ignored"
bash tools/oracle/run_duarouter.sh $CASES
echo "===== results.tsv ====="
cat ${TMPDIR:-/tmp}/duarouter_oracle/results.tsv
echo "===== diffs ====="
for c in $CASES; do
  d=${TMPDIR:-/tmp}/duarouter_oracle/$c
  for f in "$d"/*.diff; do
    [ -f "$f" ] || continue
    echo "--- $c $(basename $f) ---"
    head -40 "$f"
  done
  echo "--- $c cand.log (head) ---"
  head -20 "$d/cand.log"
done