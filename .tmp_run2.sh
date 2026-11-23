#!/bin/bash
set -u
cd /mnt/d/sumo-wt/agent-oc108 || exit 1
OUT=/mnt/d/sumo-wt/agent-oc108/.tmp_out
rm -rf "$OUT"; mkdir -p "$OUT"
OFF="$HOME/sumo-pristine/bin/polyconvert"
for c in polyconvert_itt_polyconvert_import_OSM_buildings_from_relation polyconvert_itt_polyconvert_import_OSM_inner_shape polyconvert_itt_polyconvert_import_OSM_water_incomplete; do
  args=$(tr -d '\r' < tools/oracle/cases/$c/args)
  rargs=${args//@DIR@/$OUT/$c/ref}
  mkdir -p "$OUT/$c/ref"
  $OFF $rargs > "$OUT/$c/ref.log" 2>&1; echo "$c official rc=$?"
done
