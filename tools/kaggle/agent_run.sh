#!/bin/bash
# TEMP probe (remove before final commit): official vs ours on the taxi/control_idle complex test.
set -u
OUT=/tmp/oc117
mkdir -p "$OUT"
export COMPLEX_OUT="$OUT/complex"
export COMPLEX_TIMEOUT=${COMPLEX_TIMEOUT:-120}
rm -f "$COMPLEX_OUT/vehicle/taxi/control_idle/diff.txt"
COMPLEX_FILTER="vehicle/taxi/control_idle" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -E '^(PASS|FAIL) ' | head -1
d="$COMPLEX_OUT/vehicle/taxi/control_idle/diff.txt"
if [ -f "$d" ]; then
  echo "--- first differing lines for vehicle/taxi/control_idle"
  head -40 "$d"
else
  echo "no diff file: control_idle matched"
fi

# TEMP probe (remove before final commit): allowed-lane list for ed1 during distance_after_reroute (ours only)
DR="$OUT/distance_hook"
rm -rf "$DR"; mkdir -p "$DR"; cp -r tools/oracle/complex_bundle/vehicle/distance_after_reroute/. "$DR/"
(cd "$DR" && OC_DEBUG_ALLOWED=1 SUMO_BINARY="$CARGO_TARGET_DIR/debug/sumo" NETCONVERT_BINARY="$CARGO_TARGET_DIR/debug/netconvert" PYTHONPATH="$SUMO_HOME/tools" timeout 120 python3 runner.py 2>&1 | grep -E "^OC_ALLOWED ed1 " | head -20)
