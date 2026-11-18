#!/bin/bash
# Precise state_meso diff: ours vs pristine, line-level on the state body.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
OURS=$BIN/sumo
PRISTINE=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}/sumo
OUT=/tmp/oc12-state2; rm -rf "$OUT"; mkdir -p "$OUT/ours" "$OUT/pris"
D=tools/oracle/cases/state_meso
EXTRA=$(cat "$D/sim.args" | tr -d '\r')
SO=$(sed "s#@OUT@#$OUT/ours#g" "$D/sim_outputs.txt" | tr -d '\r')
SP=$(sed "s#@OUT@#$OUT/pris#g" "$D/sim_outputs.txt" | tr -d '\r')
"$OURS" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $EXTRA $SO >"$OUT/ours.log" 2>&1
"$PRISTINE" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $EXTRA $SP >"$OUT/pris.log" 2>&1
grep -m1 -E 'panicked at|Error:' "$OUT/ours.log" | cut -c1-140
python3 tools/oracle/compare_xml.py "$OUT/pris/state_200.xml" "$OUT/ours/state_200.xml" 2>&1 | head -4
# strip XML declaration/comment/config block, then unified diff
strip() { sed -e '/^<\?xml/d' -e '/^[[:space:]]*<!--/,/^[[:space:]]*-->/d' -e '/<sumoConfiguration/,/<\/configuration>/d' "$1" | grep -v '^[[:space:]]*$'; }
strip "$OUT/pris/state_200.xml" > "$OUT/pris.body"
strip "$OUT/ours/state_200.xml" > "$OUT/ours.body"
echo "--- diff (pristine -> ours), first 60 lines ---"
diff -u "$OUT/pris.body" "$OUT/ours.body" | head -60
echo "AGENT_RUN DONE"
