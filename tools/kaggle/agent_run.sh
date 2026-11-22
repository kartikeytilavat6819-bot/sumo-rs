#!/usr/bin/env bash
# oc98 probe: run official and our sumo on the target cases, compare tripinfo.
OUT=/tmp/oc98; mkdir -p "$OUT"
OURS="${CARGO_TARGET_DIR:-/tmp/none}/debug/sumo"
OFF="${SUMO_PRISTINE_BIN:-/tmp/none}/sumo"
CASES="tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait tt_bugs_17xxx_ticket17583 tt_bugs_4xxx_ticket4299"
for c in $CASES; do
  d="tools/oracle/cases/$c"
  args=$(cat "$d/sim.args" 2>/dev/null)
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    timeout 300 "$bin" $args --tripinfo-output "$OUT/$c.$side.tripinfo.xml" > "$OUT/$c.$side.stdout" 2> "$OUT/$c.$side.stderr"
    echo "== $c $side exit=$?"
    head -n 3 "$OUT/$c.$side.stderr"
  done
  echo "-- first differing tripinfo lines:"
  diff "$OUT/$c.off.tripinfo.xml" "$OUT/$c.ours.tripinfo.xml" 2>&1 | head -n 6
done
echo "AGENT RUN DONE"
