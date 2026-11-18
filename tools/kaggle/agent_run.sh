#!/bin/bash
set -u
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
cd tools/oracle/cases/traci_basic
S="$CARGO_TARGET_DIR/debug/sumo"
for k in 0 1 2 3 4 5; do
  timeout 1300 python3 client.py --case junction --filter "h06 " --skip "h06 tl" --shard $k/6 --port $((9300+k*30)) --ours "$S" > /tmp/z24_j$k.txt 2>&1 &
done
timeout 600 python3 client.py --case signal --filter "h06 tl" --port 9490 --ours "$S" > /tmp/z24_s.txt 2>&1 &
wait
for f in /tmp/z24_j?.txt /tmp/z24_s.txt; do
echo "== $f lines=$(wc -l < $f)"
grep -E "^  \[|DIFF|TOTAL|h06 new|Traceback|Error" $f | grep -v -E "Retrying|Warn" | cut -c1-400 | head -40
done
