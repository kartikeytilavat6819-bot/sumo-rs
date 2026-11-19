#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
C=tools/oracle/cases/jtrrouter_itt_jtrrouter_function_time_window_before_begin
sub=/tmp/bb3
rm -rf "$sub"; mkdir -p "$sub"
net=$(tr -d '[:cntrl:]' < "$C/net")
args=$(tr -d '[:cntrl:]' < "$C/args")
cargs=${args//@CASE@/$C}; cargs=${cargs//@DIR@/$sub}
echo "=== CAND ==="
"$CARGO_TARGET_DIR/debug/jtrrouter" -n "$net" $cargs --no-step-log true >"$sub/out.log" 2>"$sub/err.log"
echo "exit=$?"
echo "-- stdout --"; cat "$sub/out.log"
echo "-- stderr --"; cat "$sub/err.log"