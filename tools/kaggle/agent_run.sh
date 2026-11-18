#!/bin/bash
set -u
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
cd tools/oracle/cases/traci_basic
S="$CARGO_TARGET_DIR/debug/sumo"
for k in 0; do
  timeout 1300 python3 client.py --case junction --filter "departLane=9,getFollower" --skip "h06 tl" --shard $k/1 --port $((9300+k*30)) --ours "$S" > /tmp/z24_j$k.txt 2>&1 &
done
wait
for f in /tmp/z24_j?.txt /tmp/z24_s.txt; do
echo "== $f lines=$(wc -l < $f)"
grep -E "^  \[|DIFF|TOTAL|h06 new|Traceback|Error" $f | grep -v -E "Retrying|Warn" | cut -c1-400 | head -40
done
echo "== ours log excerpts"
L="$(pwd)/traci_ours_junction.log"; ls $(pwd)/*.log | head
grep -n -E "Z24TRACE" "$L" | head -40
grep -n -E "panicked|SIGSEGV|overflow|abort|Error:" -A3 "$L" | head -60
