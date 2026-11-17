#!/bin/bash
# TEMPORARY (nx04): node-shape trace of the joined city_munich node (#29more), ours vs C++.
set -u
W=/tmp/nx04; rm -rf $W; mkdir -p $W
ARGS="--simple-projection true --junctions.join true --junctions.join-dist 20"
IN=tools/oracle/cases/city_munich/input.osm
export SUMO_HOME=/home/gate/sumo_pkg/sumo
CPPNC=$(bash tools/gcloud/vm_cpp_trace.sh nx04 tools/kaggle/nc_trace/nx04.patch netconvert) || { echo "CPP BUILD FAILED"; exit 0; }
echo "cpp netconvert: $CPPNC"
NC_JTRACE=1 $CARGO_TARGET_DIR/debug/netconvert $ARGS --osm-files $IN -o $W/ours.net.xml > $W/ours.out 2> $W/ours.err
echo "ours rc=$?"
NC_JTRACE=1 $CPPNC $ARGS --osm-files $IN -o $W/cpp.net.xml > $W/cpp.out 2> $W/cpp.err
echo "cpp rc=$?"
grep -a '^JTRACE' $W/ours.err > $W/ours.trace
grep -a '^JTRACE' $W/cpp.err > $W/cpp.trace
echo "trace lines: ours=$(wc -l < $W/ours.trace) cpp=$(wc -l < $W/cpp.trace)"
diff $W/cpp.trace $W/ours.trace | head -40 | cut -c1-400
echo "NX04 DIFF END"
