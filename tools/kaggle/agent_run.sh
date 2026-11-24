#!/bin/bash
# oc123: run the target complex tests both ways, print first differing lines.
OUT=/tmp/oc123; mkdir -p $OUT
for t in misc/concurrent person/person simulation/loadState/persons_include simulation/simulation \
  trafficlight/railsignal_state vehicle/cfmodel/setParameter vehicle/setStop/newStop_fast_forced \
  vehicle/setStop/newStop_fast_forced2 vehicle/setStop_default_state vehicle/stopDelay_waypoint; do
  COMPLEX_OUT=$OUT COMPLEX_FILTER="$t" timeout 300 bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL)' | cut -c1-300
  [ -f "$OUT/$t/diff.txt" ] && head -30 "$OUT/$t/diff.txt" | cut -c1-240
  grep -m3 -i panick "$OUT/$t/ours/console.txt" 2>/dev/null
done
