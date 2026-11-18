#!/bin/bash
# diagnostics: first diff lines + cand log head of each non-passing case (repo dir is the cwd)
for t in dfrouter activitygen; do
  TOOL=$t OUT=/tmp/zz_$t bash tools/oracle/run_dfag.sh >/dev/null 2>&1
  for d in /tmp/zz_$t/*/; do c=$(basename $d); grep -q "^$c	PASS" /tmp/zz_$t/results.tsv && continue
    echo "=== $c"; head -5 $d/cand.log | cut -c1-200; echo "-- ref"; head -3 $d/ref.log | cut -c1-200
    for f in $d*.diff; do [ -s $f ] && { echo "-- $f"; head -12 $f | cut -c1-200; }; done
  done
done
