#!/bin/bash
set -u
cd /c/Users/karti/sumo-rs/.claude/worktrees/wave2 || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-x07
export CARGO_BUILD_JOBS=4
REF=/c/Users/karti/sumo-rs/tools/oracle/cases
cargo build -q -p netconvert 2>&1 | tail -2
for c in junction roundabout signal; do
  out="/c/Users/karti/Temp/ours_$c.net.xml"
  echo "=== $c"
  "$CARGO_TARGET_DIR/debug/netconvert.exe" --osm-files "tools/oracle/cases/$c/input.osm" \
      --simple-projection -o "$out" >/tmp/nc_$c.log 2>&1
  echo "netconvert exit: $?  ($(tail -1 /tmp/nc_$c.log))"
  python tools/oracle/compare_net.py "$REF/$c/ref_simple.net.xml" "$out" >/tmp/cmp_$c.txt 2>&1
  echo "compare exit: $?"
  tail -4 /tmp/cmp_$c.txt
done
