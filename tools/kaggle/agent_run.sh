#!/bin/bash
# oc123: run simulation/simulation both ways; show where the router statistics lines go.
OUT=/tmp/oc123; mkdir -p $OUT
t=simulation/simulation
COMPLEX_OUT=$OUT COMPLEX_FILTER="$t" timeout 300 bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL)' | cut -c1-200
for side in official ours; do
  echo "== $side console.txt answered:"; grep -n "answered\|spent" $OUT/$t/$side/console.txt | cut -c1-160
  echo "== $side log.txt answered:"; grep -n "answered\|spent" $OUT/$t/$side/log.txt | cut -c1-160
  echo "== $side log.txt tail:"; tail -4 $OUT/$t/$side/log.txt | cut -c1-160
  echo "== $side console.txt tail:"; tail -3 $OUT/$t/$side/console.txt | cut -c1-160
done
grep -n "log" tools/oracle/complex_bundle/$t/runner.py | head -5
