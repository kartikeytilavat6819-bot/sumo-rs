#!/bin/bash
# Gate diagnostic for oc55: instrumented WE.1 collision-stop trace.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo"
[ -x "$SUMO" ] || SUMO="$BIN/sumo.exe"
OUT="${TMPDIR:-/tmp}/oc55-spot"
rm -rf "$OUT"; mkdir -p "$OUT" || { echo "no temp dir $OUT"; exit 1; }
c=tt_extended_collisions_intermodal_stoptime_remove
d=tools/oracle/cases/$c
flags=$(cat "$d/sim.args")
timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
    --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
    --no-step-log true >"$OUT/$c.log" 2>&1
echo "rc=$?  log lines: $(wc -l < "$OUT/$c.log")"
echo "=== DBG process_next_stop (first 40) ==="
grep 'DBG process_next_stop' "$OUT/$c.log" | head -40
echo "=== DBG process_next_stop (last 40) ==="
grep 'DBG process_next_stop' "$OUT/$c.log" | tail -40
echo "=== DBG collision_stop_time (t=45..85, dedup by t) ==="
grep 'DBG collision_stop_time' "$OUT/$c.log" | awk '{print $4, $5, $6, $7, $8, $9, $10, $11}' | sort -u -k1,1 | head -60
