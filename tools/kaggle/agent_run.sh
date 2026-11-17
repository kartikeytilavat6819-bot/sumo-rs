#!/bin/bash
# TRACE-ONLY measurement for agent nx01 (removed in the final commit). Gate VM: ours = $CARGO_TARGET_DIR, C++ = $SUMO_PRISTINE_BIN.
R=$CARGO_TARGET_DIR/debug/netconvert
C=$SUMO_PRISTINE_BIN/netconvert
E=$PWD/tools/oracle/cases/netconv_edge
O=/tmp/nx01
mkdir -p $O
for c in base_ok empty_zero empty_osm truncated missing_nodes dup_nodes dup_ways unclosed_area single_node_way unusual_tags; do
  d=$E/$c; o=$O/edge_$c; mkdir -p $o
  timeout 300 $R --simple-projection true --osm-files $d/input.osm -o $o/rust.net.xml > $o/rust.log 2>&1; r=$?
  timeout 300 $C --simple-projection true --osm-files $d/input.osm -o $o/cpp.net.xml > $o/cpp.log 2>&1; cc=$?
  echo "EDGE $c rust=$r cpp=$cc"
  diff $o/cpp.log $o/rust.log | head -8 | cut -c1-300
done
export RUST_NETCONVERT=$R CPP_NETCONVERT=$C CPP_SUMO_HOME=$SUMO_HOME NETOPTS_OUT=$O/netopts
bash tools/oracle/run_netopts.sh city_munich > $O/netopts.out 2>&1
S=$O/netopts/city_munich/summary.tsv
echo "NETOPTS city_munich: $(awk -F'\t' '$3=="PASS"' $S | wc -l) PASS / $(wc -l < $S) rows"
awk -F'\t' '$3!="PASS"' $S | cut -c1-400
