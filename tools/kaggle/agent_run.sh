#!/bin/bash
# TEMPORARY (nx03): JTRACE of join clusters, city_munich + --junctions.join-dist 20, ours vs C++.
set -u
W=/tmp/nx03; rm -rf $W; mkdir -p $W
ARGS="--simple-projection true --junctions.join true --junctions.join-dist 20"
IN=tools/oracle/cases/city_munich/input.osm
export SUMO_HOME=/home/gate/sumo_pkg/sumo
CPPNC=$(bash tools/kaggle/vm_cpp_trace_nc.sh nx03 tools/kaggle/nc_trace/NBNodeCont.cpp) || { echo "CPP BUILD FAILED"; exit 0; }
echo "cpp netconvert: $CPPNC"
NC_JTRACE=1 $CARGO_TARGET_DIR/debug/netconvert $ARGS --osm-files $IN -o $W/ours.net.xml > $W/ours.out 2> $W/ours.err
echo "ours rc=$?"
NC_JTRACE=1 $CPPNC $ARGS --osm-files $IN -o $W/cpp.net.xml > $W/cpp.out 2> $W/cpp.err
echo "cpp rc=$?"
grep -a '^JTRACE' $W/ours.err > $W/ours.trace
grep -a '^JTRACE' $W/cpp.err > $W/cpp.trace
echo "trace lines: ours=$(wc -l < $W/ours.trace) cpp=$(wc -l < $W/cpp.trace)"
sed -i 's/ true/ 1/; s/ false/ 0/' $W/ours.trace; sed -i 's/ true/ 1/; s/ false/ 0/' $W/cpp.trace
diff $W/cpp.trace $W/ours.trace | head -60
echo "first diff stage: $(diff $W/cpp.trace $W/ours.trace | head -1)"
echo "NX03 DIFF END"
