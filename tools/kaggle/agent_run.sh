#!/usr/bin/env bash
# oc112 probe: run official and Rust sumo on the expected-error rows and print
# exit codes, first stderr lines and first differing reference lines.
# Runs from the repo root on the gate VM. Writes only under /tmp/oc112.
set -u
OUT=/tmp/oc112
rm -rf "$OUT"; mkdir -p "$OUT"
ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT" || exit 1
PRISTINE="${SUMO_PRISTINE_BIN:-}/sumo"
OURS="${CARGO_TARGET_DIR:-$ROOT/target}/debug/sumo"
ROWS="$OUT/rows.txt"
# Row list is inlined: .oc_haiku_rows.txt is untracked and absent on the gate VM.
cat > "$ROWS" <<'ROWLIST'
tt_output_route_probe_devnull_output
tt_output_route_probe_null_output
tt_output_route_probe_odd_interval
tt_output_route_probe_two_same
tt_output_route_probe_two_same_b5
tt_output_route_probe_two_same_begin
tt_output_route_probe_vTypes
tt_output_tripinfo_one_vehicle_subsecond
tt_output_tripinfo_probability_by_type
tt_output_tripinfo_teleport
tt_output_tripinfo_teleport_multihop
tt_output_tripinfo_write_undeparted
tt_output_vehroutes_dua
tt_output_vehroutes_route_dist
tt_output_vehroutes_transportable_write_unfinished_person
tt_output_vehroutes_with_stops
tt_output_vehroutes_with_stops_priorEdges
tt_output_vehroutes_with_stops_started_ended
tt_output_vehroutes_write_unfinished_stops
tt_devices_ssm_config_filter_edges_with_conflict
tt_devices_ssm_errors_filter_edges_cannot_interpret_line
tt_devices_ssm_errors_filter_edges_edge_not_found
tt_devices_ssm_errors_filter_edges_junction_not_found
ROWLIST

first_diff() { # ref file, got file
  diff <(grep -v '^\s*$' "$1") <(grep -v '^\s*$' "$2") | head -3 | cut -c1-160
}

while read -r c; do
  [ -n "$c" ] || continue
  d="tools/oracle/cases/$c"
  [ -d "$d" ] || { echo "$c: missing case dir"; continue; }
  extra=""; [ -f "$d/sim.args" ] && extra=$(cat "$d/sim.args")
  for who in pristine ours; do
    bin=$PRISTINE; [ "$who" = ours ] && bin=$OURS
    timeout 600 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $extra \
      >"$OUT/$c.$who.out" 2>"$OUT/$c.$who.err"
    echo $? >"$OUT/$c.$who.rc"
  done
  rc_p=$(cat "$OUT/$c.pristine.rc"); rc_o=$(cat "$OUT/$c.ours.rc")
  echo "== $c rc official=$rc_p ours=$rc_o"
  echo "   official stderr: $(head -2 "$OUT/$c.pristine.err" | tr '\n' '|' | cut -c1-200)"
  echo "   official stdout: $(head -2 "$OUT/$c.pristine.out" | tr '
' '|' | cut -c1-200)"
  echo "   ours stdout:     $(head -2 "$OUT/$c.ours.out" | tr '
' '|' | cut -c1-200)"
  echo "   ours stderr:     $(head -2 "$OUT/$c.ours.err" | tr '\n' '|' | cut -c1-200)"
  if [ -f "$d/ref.errors" ]; then
    diff_out=$(first_diff "$d/ref.errors" "$OUT/$c.ours.err")
    [ -n "$diff_out" ] && echo "   first diff vs ref.errors: $(echo "$diff_out" | head -1)"
  fi
  if [ -f "$d/ref.output" ]; then
    diff_out=$(first_diff "$d/ref.output" "$OUT/$c.ours.out")
    [ -n "$diff_out" ] && echo "   first diff vs ref.output: $(echo "$diff_out" | head -1)"
  fi
done < "$ROWS"

# --- rail/TLS regression stage (oc112): tripinfo of both binaries against ref.tripinfo.xml
RAILROWS="$OUT/railrows.txt"
cat > "$RAILROWS" <<'RAILLIST'
tt_rail_deadlocks_loadHints_loop_4veh3
tt_rail_deadlocks_loadHints_triangle_6veh
tt_rail_deadlocks_loop_4veh3
tt_rail_rail_signal_driveway_variants
tt_rail_rail_signal_end_block_mode_end_after_bidi
tt_rail_rail_signal_end_block_mode_end_within_bidi
tt_rail_rail_signal_indirect_guard_route_conflict2
tt_rail_rail_signal_indirect_guard_route_conflict3
tt_rail_rail_signal_indirect_guard_route_conflict_reroute
tt_rail_rail_signal_moving_block_protect_reversal2
tt_rail_rail_signal_moving_block_protect_reversal_noSignal
tt_rail_rail_signal_moving_block_protect_selfintersecting
tt_rail_rail_signal_moving_block_protect_selfintersecting2
tt_rail_rail_signal_siding_deadend
tt_rail_reversal_subdriveway_reversal
tt_rail_routing_reverse_on_short_start
tt_rail_siding_long_siding_occupied_wait3
tt_rail_siding_loop_3veh
tt_rail_siding_sidingShort_deadlock
tt_rail_siding_siding_deadlock
tt_rail_siding_siding_deadlock_approaching_limit
tt_rail_siding_siding_deadlock_intermediate_dw
tt_rail_siding_siding_noSignal
tt_rail_siding_siding_noSignal2
tt_rail_tram_bidi_conflict_insertion_tls
tt_tls_NEMA_errors_customDetectorsMissing
city_melbourne_guess_signals
city_phoenix_scale_grid
jt_unreg_5arm
tt_junction_model_indirect_left_turn_internal_junction_controlled_moped
tt_sublane_model_junction_indirect_left_moped
tt_sublane_model_junction_turnAlignment_moped_indirect_left
RAILLIST
while read -r c; do
  d="tools/oracle/cases/$c"
  [ -f "$d/ref.tripinfo.xml" ] || { echo "$c: no ref"; continue; }
  EXTRA=""; [ -f "$d/sim.args" ] && EXTRA=$(cat "$d/sim.args")
  for who in pristine ours; do
    bin=$PRISTINE; [ "$who" = ours ] && bin=$OURS
    timeout 600 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.$who.ti.xml" --fcd-output "$OUT/$c.$who.fcd.xml" --statistic-output "$OUT/$c.$who.stat.xml" --no-step-log true $EXTRA >"$OUT/$c.$who.sim.log" 2>&1
    echo "$?" >"$OUT/$c.$who.simrc"
    python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.$who.ti.xml" >"$OUT/$c.$who.cmp" 2>&1
    [ -f "$d/ref.stat.xml" ] && python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/$c.$who.stat.xml" >"$OUT/$c.$who.stat.cmp" 2>&1 || true
    [ -f "$d/ref.fcd.xml" ] && python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.$who.fcd.xml" >"$OUT/$c.$who.fcd.cmp" 2>&1 || true
    echo "$?" >"$OUT/$c.$who.cmprc"
  done
  echo "== $c official rc=$(cat "$OUT/$c.pristine.simrc") cmp=$(cat "$OUT/$c.pristine.cmprc") | ours rc=$(cat "$OUT/$c.ours.simrc") cmp=$(cat "$OUT/$c.ours.cmprc")"
  [ "$(cat "$OUT/$c.ours.cmprc")" = 0 ] || echo "   ours diff: $(grep -v '^\s*$' "$OUT/$c.ours.cmp" | head -1 | cut -c1-220)"
  [ -s "$OUT/$c.ours.stat.cmp" ] && echo "   ours stat: $(grep -v '^\s*$' "$OUT/$c.ours.stat.cmp" | head -1 | cut -c1-220)"
  [ -f "$OUT/$c.ours.fcd.cmp" ] && [ -s "$OUT/$c.ours.fcd.cmp" ] && echo "   ours fcd: $(grep -v '^\s*$' "$OUT/$c.ours.fcd.cmp" | head -1 | cut -c1-220)"
done < "$RAILROWS"
