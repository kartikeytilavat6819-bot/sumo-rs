#!/bin/bash
# TEMPORARY (pn01): first differing element for the three net-plain FAIL rows. Removed in the final commit.
for c in cfmodels reroute stress; do
  d=tools/oracle/cases/$c
  $CARGO_TARGET_DIR/debug/netconvert $(cat $d/net_plain.args) -o /tmp/pn01_$c.net.xml > /tmp/pn01_$c.nc.log 2>&1
  echo "== $c netconvert rc=$?"
  grep -v -i warning /tmp/pn01_$c.nc.log | head -3
  python tools/oracle/compare_net.py $d/ref_plain.net.xml /tmp/pn01_$c.net.xml > /tmp/pn01_$c.cmp.log 2>&1
  echo "== $c compare rc=$?"
  head -12 /tmp/pn01_$c.cmp.log
done
