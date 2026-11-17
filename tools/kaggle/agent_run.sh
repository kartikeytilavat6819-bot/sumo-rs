#!/bin/bash
# temporary (sl05): where does the routing engine first init edge weights?
cd "$(dirname "$0")/../.." || exit 1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
cd tools/oracle/cases/routes_forms || exit 1
SL05_TRACE=1 RUST_BACKTRACE=0 $BIN -n ref_simple.net.xml -r routes.rou --end 100 --no-step-log true --save-state.times 50 --save-state.files /tmp/sl05_rf.state.xml >/tmp/sl05_rf.log 2>&1
grep -n "SL05 init" /tmp/sl05_rf.log | head -5
grep -A40 -m1 "SL05 init" /tmp/sl05_rf.log | grep -E "sumo_core|sumo::|  at " | head -40
echo "AGENT RUN DONE"
