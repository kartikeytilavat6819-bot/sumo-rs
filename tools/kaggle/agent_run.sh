#!/usr/bin/env bash
# oc102 probe: persons_include state.xml at t=1, ours vs official, p1 line.
OUT=/tmp/oc102; mkdir -p "$OUT/ours" "$OUT/ref"
CASE=tools/oracle/complex_bundle/simulation/loadState/persons_include
for side in ours ref; do
  if [ $side = ours ]; then BIN="$CARGO_TARGET_DIR/debug/sumo"; D="$OUT/ours"; else BIN="$SUMO_PRISTINE_BIN/sumo"; D="$OUT/ref"; fi
  (cd "$CASE" && "$BIN" -n input_net2.net.xml -a input_routes.rou.xml --save-state.transportables --save-state.times 1 --save-state.prefix "$D/st" --no-step-log -e 2 >/dev/null 2>"$D/err.txt")
  echo "--- $side"; grep -h 'person id="p1"' "$D"/st*.xml 2>/dev/null | cut -c1-400
done
echo "AGENT RUN DONE"
