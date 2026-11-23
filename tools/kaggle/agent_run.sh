#!/usr/bin/env bash
# oc102 probe: ticket4299 with our binary; print ES1 link-approach probe and collision warnings.
OUT=/tmp/oc102; mkdir -p "$OUT"
cd tools/oracle/cases/tt_bugs_4xxx_ticket4299 || exit 1
"$CARGO_TARGET_DIR/debug/sumo" $(cat sim.args) -n net.net.xml -r input_routes.rou.xml --tripinfo-output "$OUT/ours.tripinfo.xml" 2> "$OUT/ours.err" >/dev/null
echo "ours rc=$?"
grep -e "collision" -e "OC102I" -e "OC102L" -e "OC102J" "$OUT/ours.err" | head -200
echo "AGENT RUN DONE"
