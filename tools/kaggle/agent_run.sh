#!/bin/bash
set -u
cd /c/Users/karti/sumo-rs 2>/dev/null || cd "$(dirname "$0")/../.."
ROOT=$(pwd)
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}
export SUMO_PRISTINE_BIN=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}
OUT=/tmp/oc111
rm -rf "$OUT"; mkdir -p "$OUT"
C=tools/oracle/cases/itt_nc_netconvert_function_railway_signals_guess_by_stops_split
ARGS="--write-license --edge-files=$ROOT/$C/input_plain.edg.xml --node-files=$ROOT/$C/input_plain.nod.xml --ptstop-files $ROOT/$C/input_ptstops.xml --railway.signal.guess.by-stops --railway.signal.guess.by-stops.split"
"$CARGO_TARGET_DIR/debug/netconvert" $ARGS --ptstop-output "$OUT/rust_additional.xml" -o "$OUT/rust.net.xml" > "$OUT/rust.log" 2>&1
echo "rust rc=$?"
"$SUMO_PRISTINE_BIN/netconvert" $ARGS --ptstop-output "$OUT/cpp_additional.xml" -o "$OUT/cpp.net.xml" > "$OUT/cpp.log" 2>&1
echo "cpp rc=$?"
echo "===== compare_net diff (first 40) ====="
python3 tools/oracle/compare_net.py "$OUT/cpp.net.xml" "$OUT/rust.net.xml" > "$OUT/net.diff" 2>&1
head -40 "$OUT/net.diff"
echo "===== additional.xml diff ====="
diff "$OUT/cpp_additional.xml" "$OUT/rust_additional.xml" | head -40
echo "===== cpp.log tail ====="
tail -20 "$OUT/cpp.log"
echo "===== rust.log tail ====="
tail -20 "$OUT/rust.log"
