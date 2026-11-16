#!/bin/bash
# Case 7: real/input.osm under option variants. Logs and nets go to netconv_edge/real_<variant>/.
REAL=/mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-rb06/tools/oracle/cases/real
E=/mnt/c/Users/karti/sumo-rs/.claude/worktrees/agent-rb06/tools/oracle/cases/netconv_edge
RUST=/mnt/c/Users/karti/sumo-rs-target-main/debug/netconvert.exe
CPP=$HOME/sumo-src/bin/netconvert
run_variant() {
  v=$1; shift
  d=$E/real_$v
  mkdir -p $d
  wd=$(wslpath -w "$d")
  wr=$(wslpath -w "$REAL")
  rm -f $d/rust.net.xml $d/cpp.net.xml
  timeout 900 "$RUST" --simple-projection true "$@" --osm-files "$wr\\input.osm" -o "$wd\\rust.net.xml" > "$d/rust.log" 2>&1
  echo "real_$v rust_exit=$?"
  SUMO_HOME=$HOME/sumo-src timeout 900 "$CPP" --simple-projection true "$@" --osm-files "$REAL/input.osm" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1
  echo "real_$v cpp_exit=$?"
}
run_variant base
run_variant noturn --no-turnarounds true
run_variant corner3 --junctions.corner-detail 3
