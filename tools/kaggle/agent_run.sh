#!/bin/bash
# pv03 temporary: TraCI probes for actuated + NEMA parameters and program switching. Removed in the final commit.
export SUMO_HOME="${SUMO_HOME:-$HOME/sumo-pristine/lib/python3.12/site-packages/sumo}"
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
export SUMO_PRISTINE_BIN="${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}"
OURS="$CARGO_TARGET_DIR/debug/sumo"
C=tools/oracle/cases/traci_nema
echo "=== NEMA net, pristine C++ alone"
timeout 30 "$SUMO_PRISTINE_BIN/sumo" -n $C/ref_simple.net.xml -r $C/routes.rou --end 5 --no-step-log true 2>&1 | head -8
echo "=== NEMA net, ours alone"
timeout 30 "$OURS" -n $C/ref_simple.net.xml -r $C/routes.rou --end 5 --no-step-log true 2>&1 | head -8
for c in traci_nema; do
  echo "=== PROBE $c"
  timeout 900 python3 tools/oracle/cases/traci_basic/client.py --case "$c" --ours "$OURS" --port 9800 > /tmp/pv03_nema.txt 2>&1
  grep -E "tl param|MISMATCH|TOTAL|Error|error" /tmp/pv03_nema.txt | cut -c1-260 | head -40
done
echo "AGENT RUN DONE"
