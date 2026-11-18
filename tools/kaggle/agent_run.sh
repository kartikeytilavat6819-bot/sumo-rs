#!/bin/bash
set -u
export PYTHONPATH="$SUMO_HOME/tools:${PYTHONPATH:-}"
cd tools/oracle/cases/traci_basic
for c in junction signal; do
echo "== all probes case $c"
timeout 1500 python3 client.py --case $c --port 9300 --ours "$CARGO_TARGET_DIR/debug/sumo" > /tmp/z24_$c.txt 2>&1
grep -E "DIFF|NEW-PROBE|^  \[|TOTAL|h06 new|InitFailed|Traceback" /tmp/z24_$c.txt | cut -c1-330 | head -120
done
