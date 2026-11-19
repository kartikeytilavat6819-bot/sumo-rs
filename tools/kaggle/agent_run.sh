#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
D=C:/Users/karti/sumo-src/tests/jtrrouter/function/time_window/before_begin
sub=/tmp/before_begin_orig
rm -rf "$sub"; mkdir -p "$sub"
cd "$D"
echo "=== pristine ==="
"$SUMO_PRISTINE_BIN/jtrrouter" --net-file=input_net.net.xml --route-files=input_flows.flows.xml --output-file="$sub/routes.rou.xml" --sinks=end --begin=20 --end=30 --no-step-log true >"$sub/ref.log" 2>&1
echo "exit=$?"
cat "$sub/ref.log"
echo "=== files ==="; ls -la "$sub"