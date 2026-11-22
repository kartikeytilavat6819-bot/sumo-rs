#!/usr/bin/env bash
# oc104 probe: official vs ours on the target rows (SSM, rerouting A* landmark, taxi).
# Each side runs inside a copy of the case dir (TextTest cwd; relative inputs such as
# lookuptable resolve there). sim.args is split with shlex, so quoted values survive.
# Outputs under /tmp/oc104. Prints exit codes, first stderr lines, first diffs vs refs.
OUT=/tmp/oc104; mkdir -p "$OUT"
ROOT=$(pwd)
OFF="${SUMO_PRISTINE_BIN:?}/sumo"
OURS="${CARGO_TARGET_DIR:-target}/debug/sumo"
ROWS="
tt_devices_rerouting_assignment_taz_astar_landmark
tt_devices_rerouting_assignment_taz_astar_landmark_empty
tt_devices_rerouting_assignment_taz_astar_landmark_missing
tt_devices_rerouting_parallel_taz_astar_landmark_parallel
tt_devices_ssm_bugs_ticket5231
tt_devices_ssm_config_filter_edges_with_conflict
tt_devices_ssm_errors_filter_edges_cannot_interpret_line
tt_devices_ssm_errors_filter_edges_edge_not_found
tt_devices_ssm_errors_filter_edges_junction_not_found
tt_devices_taxi_bugs_ticket9812_railRouter
tt_devices_taxi_bugs_ticket9812_taxi_vclasses
"
for c in $ROWS; do
  d="$ROOT/tools/oracle/cases/$c"
  [ -d "$d" ] || { echo "== $c: missing case dir"; continue; }
  mapfile -t EXTRA < <(python -c 'import shlex,sys; print("\n".join(a.replace(sys.argv[2], "") for a in shlex.split(open(sys.argv[1]).read())))' "$d/sim.args" "tools/oracle/cases/$c/" 2>/dev/null)
  for side in off ours; do
    bin=$OFF; [ $side = ours ] && bin=$OURS
    rdir="$OUT/$c.$side"; rm -rf "$rdir"; mkdir -p "$rdir"; cp -r "$d/." "$rdir/"
    (cd "$rdir" && timeout 600 "$bin" -n ref_simple.net.xml -r routes.rou --no-step-log true "${EXTRA[@]}" \
        >out.txt 2>err.txt; echo $? > rc.txt)
  done
  echo "== $c: exit off=$(cat "$OUT/$c.off/rc.txt") ours=$(cat "$OUT/$c.ours/rc.txt")"
  echo "  -- off stderr:"; head -n 4 "$OUT/$c.off/err.txt" | cut -c1-200 | sed 's/^/     /'
  echo "  -- ours stderr:"; head -n 4 "$OUT/$c.ours/err.txt" | cut -c1-200 | sed 's/^/     /'
  if [ -f "$d/ref.errors" ]; then
    if diff -q "$d/ref.errors" "$OUT/$c.ours/err.txt" >/dev/null; then echo "  ref.errors: MATCH"
    else echo "  ref.errors vs ours (first diff):"; diff "$d/ref.errors" "$OUT/$c.ours/err.txt" | head -n 6 | cut -c1-200 | sed 's/^/     /'; fi
  fi
  if [ -f "$d/ref.output" ]; then
    if diff -q "$d/ref.output" "$OUT/$c.ours/out.txt" >/dev/null; then echo "  ref.output: MATCH"
    else echo "  ref.output vs ours (first diff):"; diff "$d/ref.output" "$OUT/$c.ours/out.txt" | head -n 6 | cut -c1-200 | sed 's/^/     /'; fi
  fi
done
