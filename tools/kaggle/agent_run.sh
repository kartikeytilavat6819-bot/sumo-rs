#!/bin/bash
# Probe the 4 known-failing complex TraCI tests: official vs ours, print the first differing lines.
# Run from the repo root on the gate VM after the build (CARGO_TARGET_DIR, SUMO_PRISTINE_BIN, SUMO_HOME set).
set -u
OUT=/tmp/oc117
mkdir -p "$OUT"
export COMPLEX_OUT="$OUT/complex"
export COMPLEX_TIMEOUT=${COMPLEX_TIMEOUT:-120}
for t in misc/concurrent person/person simulation/loadState/persons_include simulation/simulation trafficlight/railsignal_state vehicle/cfmodel/setParameter vehicle/distance_after_reroute vehicle/insertStop/second_unreachable_ignore_rerouter vehicle/insertStop/stop_forbidden_ignore_rerouter_copy vehicle/keepAfterArrival vehicle/keepAfterArrival_state vehicle/replaceStop/first_parameter_only vehicle/setStop/newStop_fast_forced vehicle/setStop/newStop_fast_forced2; do
  COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL) ' | head -1
  d="$COMPLEX_OUT/$t/diff.txt"
  if [ -f "$d" ]; then
    echo "--- first differing lines for $t"
    head -40 "$d"
  fi
done
