#!/bin/bash
# st04 temporary diagnostics: panic locations for busride and the rerouteTraveltime probe. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export RUST_BACKTRACE=1
echo "== busride"
"$CARGO_TARGET_DIR/debug/sumo" -n tools/oracle/cases/busride/ref_simple.net.xml -r tools/oracle/cases/busride/routes.rou \
    --no-step-log true $(cat tools/oracle/cases/busride/sim.args) --tripinfo-output /tmp/br.ti.xml > /tmp/br.log 2>&1
echo "exit=$?"; grep -m3 -E "panicked|Error" /tmp/br.log | cut -c1-300
grep -A12 "stack backtrace" /tmp/br.log | grep -E "sumo_core|at " | head -20
echo "== rerouteTraveltime probe"
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
python3 tools/oracle/cases/traci_basic/client.py --port 9700 --ours "$CARGO_TARGET_DIR/debug/sumo" \
    --filter "rerouteTraveltime(v0) between" --only ours 2>/dev/null | head -5
grep -h -A14 "stack backtrace" traci_ours_junction.log 2>/dev/null | grep -E "sumo_core|overflow" | head -12
grep -h -B2 -A2 "overflow" traci_ours_junction.log 2>/dev/null | head -8
