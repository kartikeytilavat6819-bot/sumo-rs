#!/bin/bash
# sc01 temporary: show the duarouter diffs of the failing h05 cases
for c in duarouter_flows_randomize duarouter_remove_loops duarouter_vtype_distributions duarouter_write_trips; do
  OUT=/tmp/sc01_dua bash tools/oracle/run_duarouter.sh $c 2>&1 | tail -2
  for f in /tmp/sc01_dua/$c/*.diff; do [ -s "$f" ] && { echo "== $f"; head -30 "$f"; }; done
  grep -m3 -iE 'panicked|error' /tmp/sc01_dua/$c/cand.log
done
