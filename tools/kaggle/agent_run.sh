#!/bin/bash
# oc108: measure the 9 reported regression rows (official vs ours).
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc108regr
rm -rf "$OUT"; mkdir -p "$OUT"
for c in \
  tt_devices_ssm_config_filter_edges_with_conflict \
  tt_devices_ssm_errors_filter_edges_cannot_interpret_line \
  tt_devices_ssm_errors_filter_edges_edge_not_found \
  tt_devices_ssm_errors_filter_edges_junction_not_found \
  tt_output_vehroutes_with_stops \
  tt_output_vehroutes_with_stops_priorEdges \
  tt_output_vehroutes_with_stops_started_ended \
  tt_output_vehroutes_write_unfinished_stops; do
  D=tools/oracle/cases/$c
  echo "==== $c"
  for side in ref cand; do
    if [ "$side" = ref ]; then BIN="$SUMO_PRISTINE_BIN/sumo"; else BIN="$CARGO_TARGET_DIR/debug/sumo"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true \
      $(tr -d '\r' < "$D/sim.args") >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.log"
    echo " $side rc=$? stderr_lines=$(wc -l <"$OUT/$c.$side.log")"
  done
  diff -u "$OUT/$c.ref.log" "$OUT/$c.cand.log" | head -12
done
echo AGENT_RUN_DONE
