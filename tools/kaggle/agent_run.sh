#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
run() { echo "===== $1 ====="; eval "$2" >/tmp/_r.txt 2>&1; tail -3 /tmp/_r.txt; echo "-- failing (max 25) --"; grep -iE 'FAIL|ERROR|PANIC|REFERR' /tmp/_r.txt | head -25; }
run "od2trips" "bash tools/oracle/run_odtool.sh od2trips"
run "marouter" "bash tools/oracle/run_odtool.sh marouter"
run "jtrrouter" "bash tools/oracle/run_jtrrouter.sh"
run "dfrouter" "TOOL=dfrouter bash tools/oracle/run_dfag.sh"
run "activitygen" "TOOL=activitygen bash tools/oracle/run_dfag.sh"
# polyconvert/netgenerate imported cases via odtool
run "polyconvert" "bash tools/oracle/run_odtool.sh polyconvert"
run "netgenerate" "bash tools/oracle/run_odtool.sh netgenerate"
