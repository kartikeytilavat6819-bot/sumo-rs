#!/bin/bash
# st04 temporary diagnostics: backtrace of the junction rerouteTraveltime-between-stops probe. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
export RUST_BACKTRACE=1
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
rm -f traci_ours_junction.log
python3 tools/oracle/cases/traci_basic/client.py --port 9700 --ours "$CARGO_TARGET_DIR/debug/sumo" \
    --case junction --filter "rerouteTraveltime(v0) between" --only ours > /tmp/st04_probe.out 2>&1
cat /tmp/st04_probe.out | cut -c1-300 | head -20
L=$(ls "${TEMP:-.}"/traci_ours_junction.log traci_ours_junction.log 2>/dev/null | head -1)
[ -n "$L" ] || L=/dev/null
echo "log=$L"
grep -n -m2 -E "panicked|overflow" "$L" | cut -c1-300
grep -E "sumo_core|^ +[0-9]+: " "$L" | grep -v "rustc/" | head -40 | cut -c1-260
