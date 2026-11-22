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
