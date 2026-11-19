#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
C=tools/oracle/cases/jtrrouter_itt_jtrrouter_function_time_window_before_begin
sub=/tmp/bb2
rm -rf "$sub"; mkdir -p "$sub"
net=$(tr -d '[:cntrl:]' < "$C/net")
echo "=== pristine original-style ==="
"$SUMO_PRISTINE_BIN/jtrrouter" -n "$net" --route-files="$C/input_flows.flows.xml" --output-file="$sub/routes.rou.xml" --sinks=end --begin=20 --end=30 >"$sub/ref.log" 2>&1
echo "exit=$?"
cat "$sub/ref.log"
echo "=== files ==="; ls "$sub"