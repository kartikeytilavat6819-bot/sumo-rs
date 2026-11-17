#!/bin/bash
# TEMPORARY diagnostics for agent pt01 (removed before merge)
set -u
B=${CARGO_TARGET_DIR:-/tmp/target}/debug
P=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}
D=/tmp/ptdiag; rm -rf $D; mkdir -p $D
C=tools/oracle/cases
echo "P=$P B=$B"; ls $P | head -3
c=city_zurich_tram
for a in "--ptline-clean-up true" "--ptline-clean-up=true" "--ptline-clean-up" "--railway.topology.repair=true" "--railway.topology.repair"; do
  o=$D/t$(echo "$a" | tr -dc 'a-z' | cut -c1-30); mkdir -p $o
  echo "== ours: $a"
  $B/netconvert --osm-files $C/$c/input.osm --simple-projection $a -o $o/net.xml > $o/log 2>&1 </dev/null
  echo "rc=$?"; head -c 700 $o/log; echo
done
echo "== pristine ptline (full log)"
mkdir -p $D/cpp
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo $P/netconvert --osm-files $C/$c/input.osm --simple-projection --ptstop-output $D/cpp/ptstop.xml --ptline-output $D/cpp/ptline.xml --ptline-clean-up true -o $D/cpp/net.xml > $D/cpp/log 2>&1 </dev/null
echo "rc=$?"; head -c 700 $D/cpp/log; echo
echo "== ours ptline full log"
$B/netconvert --osm-files $C/$c/input.osm --simple-projection --ptstop-output $D/cpp/o.ptstop.xml --ptline-output $D/cpp/o.ptline.xml --ptline-clean-up true -o $D/cpp/o.net.xml > $D/cpp/olog 2>&1 </dev/null
echo "rc=$?"; head -c 900 $D/cpp/olog; echo
