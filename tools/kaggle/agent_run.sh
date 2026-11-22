#!/bin/bash
# oc107 measurement: official vs ours on two route-parsing error rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc107
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo.exe"
OFF="$SUMO_PRISTINE_BIN/sumo"
export SUMO_HOME="${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}"
for c in tt_basic_routes_parsing_nested_ref tt_basic_routes_parsing_repeat_stops_no_period; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  echo "----- ref.errors (expected) -----"; cat "$D/ref.errors"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      --tripinfo-output "$OUT/$c.$side.ti.xml" \
      >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"
    echo "----- $side exit=$? -----"
    echo "$side stderr:"; cat "$OUT/$c.$side.err"
    echo "$side tripinfo ids:"; grep -o '<tripinfo id="[^"]*"' "$OUT/$c.$side.ti.xml" 2>/dev/null | head -20
  done
done
echo AGENT_RUN_DONE
