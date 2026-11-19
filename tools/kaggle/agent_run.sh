#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
C=tools/oracle/cases/jtrrouter_itt_jtrrouter_function_time_window_before_begin
sub=/tmp/jtr_before2
rm -rf "$sub"; mkdir -p "$sub/ref" "$sub/cand"
net=$(tr -d '[:cntrl:]' < "$C/net")
args=$(tr -d '[:cntrl:]' < "$C/args")
rargs=${args//@CASE@/$C}; rargs=${rargs//@DIR@/$sub/ref}
cargs=${args//@CASE@/$C}; cargs=${cargs//@DIR@/$sub/cand}
echo "=== REF LOG ==="
"$SUMO_PRISTINE_BIN/jtrrouter" -n "$net" $rargs --no-step-log true >"$sub/ref.log" 2>&1
echo "exit=$?"
cat "$sub/ref.log"
echo "=== CAND LOG ==="
"$CARGO_TARGET_DIR/debug/jtrrouter" -n "$net" $cargs --no-step-log true >"$sub/cand.log" 2>&1
echo "exit=$?"
cat "$sub/cand.log"
echo "=== files ref ==="; ls "$sub/ref"
echo "=== files cand ==="; ls "$sub/cand"