#!/bin/bash
for c in duarouter_persontrip_walkfactor duarouter_intermodal_outputs; do
echo "== $c"
d=tools/oracle/cases/$c
a=$(cat $d/args); a=${a//@CASE@/$d}; a=${a//@DIR@//tmp/z35o/$c}; mkdir -p /tmp/z35o/$c
RUST_BACKTRACE=1 $CARGO_TARGET_DIR/debug/duarouter -n $(cat $d/net) $a --no-step-log true 2>&1 | grep -A40 -E "panicked|Error" | grep -E "panicked|Error|sumo_core|at \./" | head -20
ls /tmp/z35o/$c
done
