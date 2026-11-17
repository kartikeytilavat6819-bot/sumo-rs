#!/bin/bash
# TEMPORARY diagnostics for agent pt01 (removed before merge)
set -u
B=${CARGO_TARGET_DIR:-/tmp/target}/debug
D=/tmp/ptdiag; rm -rf $D; mkdir -p $D
C=tools/oracle/cases
echo "== bytes of net_outputs.txt"
od -c $C/city_zurich_tram/net_outputs.txt | head -12
for c in city_zurich_tram; do
  while read -r v args; do
    [ -n "$v" ] || continue
    o=$D/$c.$v; mkdir -p $o/r
    echo "== $c $v args=[$args]"
    $B/netconvert --osm-files $C/$c/input.osm --simple-projection ${args//@OUT@/$o/r} -o $o/r/net.net.xml > $o/log 2>&1 </dev/null
    echo "rc=$?"
    grep -v '^Warning' $o/log | head -8 | cut -c1-240
  done < $C/$c/net_outputs.txt
done
echo "== direct railway"
$B/netconvert --osm-files $C/city_zurich_tram/input.osm --simple-projection --railway.topology.repair true -o $D/rr.net.xml > $D/rr.log 2>&1 </dev/null
echo "rc=$?"; grep -v '^Warning' $D/rr.log | head -8 | cut -c1-240
