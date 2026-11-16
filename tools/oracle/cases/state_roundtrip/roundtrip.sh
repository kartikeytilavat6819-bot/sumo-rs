#!/bin/bash
# State round trip: straight run vs. save at t=10 + load in a fresh process.
# Usage: bash tools/oracle/cases/state_roundtrip/roundtrip.sh <bin_dir> <out_dir>
#   <bin_dir> holds sumo and netconvert (ours: target/debug, reference: ~/sumo-pristine/bin)
set -u
BIN=$1; OUT=$2
ROOT=$(cd "$(dirname "$0")/../../../.." && pwd)
SAVE_T=10
for c in junction signal; do
  d=$ROOT/tools/oracle/cases/$c
  o=$OUT/$c; mkdir -p "$o"
  net=$d/ref_simple.net.xml; rou=$d/routes.rou
  "$BIN/sumo" -n "$net" -r "$rou" --no-step-log true --tripinfo-output "$o/straight.ti.xml" >"$o/straight.log" 2>&1
  echo "$c straight exit=$?"
  "$BIN/sumo" -n "$net" -r "$rou" --no-step-log true --save-state.times $SAVE_T --save-state.files "$o/s.xml" \
      --tripinfo-output "$o/save.ti.xml" >"$o/save.log" 2>&1
  echo "$c save exit=$?"
  "$BIN/sumo" -n "$net" -r "$rou" --no-step-log true --load-state "$o/s.xml" --tripinfo-output "$o/load.ti.xml" \
      >"$o/load.log" 2>&1
  echo "$c load exit=$?"
  python "$ROOT/tools/oracle/cases/state_roundtrip/compare.py" tripinfo "$o/straight.ti.xml" "$o/load.ti.xml" $SAVE_T \
      >"$o/tripinfo.txt" 2>&1
  cat "$o/tripinfo.txt"
done
