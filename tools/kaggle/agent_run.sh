#!/usr/bin/env bash
# oc102 probe: run ticket4299 with ours and the official binary, print collision warnings.
OUT=/tmp/oc102; mkdir -p "$OUT"
CASE=tools/oracle/cases/tt_bugs_4xxx_ticket4299
cd "$CASE" || exit 1
ARGS=$(cat sim.args)
"$CARGO_TARGET_DIR/debug/sumo" $ARGS -n net.net.xml -r input_routes.rou.xml --tripinfo-output "$OUT/ours.tripinfo.xml" 2> "$OUT/ours.err" >/dev/null
echo "ours rc=$?"
"$SUMO_PRISTINE_BIN/sumo" $ARGS -n net.net.xml -r input_routes.rou.xml --tripinfo-output "$OUT/ref.tripinfo.xml" 2> "$OUT/ref.err" >/dev/null
echo "ref rc=$?"
echo "--- ours"; grep -i "collision" "$OUT/ours.err" | head -20
echo "--- ref";  grep -i "collision" "$OUT/ref.err" | head -20
echo "--- tripinfo diff"; diff <(grep -o 'tripinfo id="[^"]*" depart="[^"]*"[^>]*arrival="[^"]*"' "$OUT/ref.tripinfo.xml") <(grep -o 'tripinfo id="[^"]*" depart="[^"]*"[^>]*arrival="[^"]*"' "$OUT/ours.tripinfo.xml") | head -20
echo "AGENT RUN DONE"
