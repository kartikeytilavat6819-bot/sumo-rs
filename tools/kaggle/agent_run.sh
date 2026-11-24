#!/usr/bin/env bash
# oc98 probe (temporary): action-step-length warning text, official vs ours, same args as run_all.sh
OUT=/tmp/oc98; mkdir -p "$OUT"
for c in tt_errors_action_step_fractional_warning tt_errors_action_step_negative_warning; do
  d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    timeout 120 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.$side.out" 2> "$OUT/$c.$side.err"
  done
  for side in off ours; do echo "-- $c $side stderr:"; head -n 6 "$OUT/$c.$side.err" | cut -c1-260 | sed 's/^/     /'; done
  echo "-- $c stored ref.errors:"; head -n 6 "$d/ref.errors" | cut -c1-260 | sed 's/^/     /'
done
echo "AGENT RUN DONE"
