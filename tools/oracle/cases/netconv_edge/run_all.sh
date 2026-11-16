#!/bin/bash
# Runs C++ and Rust netconvert on every netconv_edge case with --simple-projection true (Rust has no PROJ binding).
# Writes <case>/{cpp,rust}.{log,net.xml} and an exit-code summary.
# Paths derive from this script's location so the runner works from any worktree.
B=$(cd "$(dirname "$0")" && pwd)
RUST=${RUST_NETCONVERT:-/mnt/c/Users/karti/sumo-rs-target-main/debug/netconvert.exe}
CPP=$HOME/sumo-src/bin/netconvert
OPTS="--simple-projection true"
run_case() {
  c=$1; shift
  d=$B/$c
  wd=$(wslpath -w "$d")
  rm -f $d/rust.net.xml $d/cpp.net.xml
  "$RUST" $OPTS "$@" --osm-files "$wd\\input.osm" -o "$wd\\rust.net.xml" > "$d/rust.log" 2>&1
  r=$?
  echo "$c rust_exit=$r"
  SUMO_HOME=$HOME/sumo-src "$CPP" $OPTS "$@" --osm-files "$d/input.osm" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  echo "$c cpp_exit=$?"
  # Compare the generated networks element by element when both sides wrote one. A raw diff is
  # useless here: the header carries the generation stamp and the recorded input/output paths.
  if [ -s $d/rust.net.xml ] && [ -s $d/cpp.net.xml ]; then
    if python3 $(dirname "$0")/../../compare_net.py $d/cpp.net.xml $d/rust.net.xml > $d/net.log 2>&1; then
      echo "$c net=MATCH"
    else
      echo "$c net=DIFF $(grep -m1 SUMMARY $d/net.log)"
    fi
  else
    echo "$c net=skip"
  fi
}
for c in base_ok empty_zero empty_osm truncated missing_nodes dup_nodes dup_ways unclosed_area single_node_way unusual_tags; do
  run_case $c
done
