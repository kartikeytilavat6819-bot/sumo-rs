#!/bin/bash
# Runs C++ and Rust netconvert on every netconv_edge case with --simple-projection true (Rust has no PROJ binding).
# Writes <case>/{cpp,rust}.{log,net.xml} and an exit-code summary.
B=/mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-rb06/tools/oracle/cases/netconv_edge
RUST=/mnt/c/Users/karti/sumo-rs-target-main/debug/netconvert.exe
CPP=$HOME/sumo-src/bin/netconvert
OPTS="--simple-projection true"
run_case() {
  c=$1; shift
  d=$B/$c
  wd=$(wslpath -w "$d")
  rm -f $d/rust.net.xml $d/cpp.net.xml
  "$RUST" $OPTS "$@" --osm-files "$wd\\input.osm" -o "$wd\\rust.net.xml" > "$d/rust.log" 2>&1
  echo "$c rust_exit=$?"
  SUMO_HOME=$HOME/sumo-src "$CPP" $OPTS "$@" --osm-files "$d/input.osm" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  echo "$c cpp_exit=$?"
}
for c in base_ok empty_zero empty_osm truncated missing_nodes dup_nodes dup_ways unclosed_area single_node_way unusual_tags; do
  run_case $c
done
