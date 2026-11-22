#!/bin/bash
# Probe the real regression: OSM parser netconvert row, official vs ours.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc100_osm
rm -rf "$OUT"
mkdir -p "$OUT"
C=tools/oracle/cases/itt_nc_netconvert_import_OSM_osm_parser
SRC=(--write-license --osm-files "$C/osm.xml" -v --proj.utm --ignore-errors --output.street-names)
"$SUMO_PRISTINE_BIN/netconvert" "${SRC[@]}" -o "$OUT/cpp.net.xml" > "$OUT/cpp.log" 2>&1
echo "official exit=$?"
"$CARGO_TARGET_DIR/debug/netconvert" "${SRC[@]}" -o "$OUT/rust.net.xml" > "$OUT/rust.log" 2>&1
echo "ours exit=$?"
echo "===== stderr diff"
diff -u "$OUT/cpp.log" "$OUT/rust.log" | head -80
echo "===== net diff"
diff -u "$OUT/cpp.net.xml" "$OUT/rust.net.xml" | head -60
