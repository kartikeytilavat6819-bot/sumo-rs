#!/bin/bash
c=duarouter_persontrip_walkfactor
d=tools/oracle/cases/$c
a=$(cat $d/args); a=${a//@CASE@/$d}; a=${a//@DIR@//tmp/z35o}; mkdir -p /tmp/z35o
RUST_BACKTRACE=1 $CARGO_TARGET_DIR/debug/duarouter -n $(cat $d/net) $a --no-step-log true 2>&1 | grep -A60 panicked | grep -E "panicked|sumo_core|utils::" | head -30
