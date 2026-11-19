#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export RUST_BACKTRACE=1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/duarouter
c=duarouter_itt_duarouter_personFlow_walk_poisson
cdir=tools/oracle/cases/$c
OUT=${TMPDIR:-/tmp}/duarouter_oracle
sub=$OUT/$c
rm -rf "$sub"; mkdir -p "$sub/cand"
net=$(cat "$cdir/net")
args=$(cat "$cdir/args")
cargs=${args//@CASE@/$cdir}; cargs=${cargs//@DIR@/$sub/cand}
echo "=== running candidate ==="
"$BIN" -n "$net" $cargs --no-step-log true
echo "exit=$?"
