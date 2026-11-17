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
P=$O/probe; mkdir -p $P
printf '<?xml version="1.0"?>\n<osm version="0.6">\n' > $P/eof_open.osm
printf '<?xml version="1.0"?>\n' > $P/eof_decl.osm
printf '<?xml version="1.0"?>\n<osm version="0.6">\n  <node id="1" lat="48" lon="11"/>\n' > $P/eof_after_node.osm
printf '<osm version="0.6">\n  <node id="1" lat=' > $P/eof_mid_attr.osm
printf '<osm version="0.6">\n  <node id="1"' > $P/eof_mid_tag.osm
printf '<osm version="0.6">\n  <node id="1"></osm>\n' > $P/mismatch.osm
printf '<osm version="0.6"/>\njunk\n' > $P/text_after_root.osm
printf '<osm version="0.6"/><osm/>\n' > $P/two_roots.osm
printf 'not xml at all\n' > $P/garbage.osm
printf '<osm version="0.6"><node id="1" lat="48" lon="11" foo="&bar;"/></osm>\n' > $P/bad_entity.osm
printf '<osm version="0.6">\n  &bar;\n</osm>\n' > $P/bad_entity_text.osm
printf '<osm version="0.6">\n  <node id="1" lat="48" lon="11"/>\n</osm>\n' > $P/ok_small.osm
for f in $P/*.osm; do
  b=$(basename $f .osm)
  timeout 300 $R --simple-projection true --osm-files $f -o $P/$b.rust.net.xml > $P/$b.rust.log 2>&1; r=$?
  timeout 300 $C --simple-projection true --osm-files $f -o $P/$b.cpp.net.xml > $P/$b.cpp.log 2>&1; cc=$?
  echo "PROBE $b rust=$r cpp=$cc"
  echo "  cpp:  $(head -c 400 $P/$b.cpp.log | tr '\n' '|')"
  echo "  rust: $(head -c 400 $P/$b.rust.log | tr '\n' '|')"
done
export RUST_NETCONVERT=$R CPP_NETCONVERT=$C CPP_SUMO_HOME=$SUMO_HOME NETOPTS_OUT=$O/netopts
bash tools/oracle/run_netopts.sh city_munich > $O/netopts.out 2>&1
S=$O/netopts/city_munich/summary.tsv
echo "NETOPTS city_munich: $(awk -F'\t' '$3=="PASS"' $S | wc -l) PASS / $(wc -l < $S) rows"
cut -f1-4 $S | cut -c1-220
