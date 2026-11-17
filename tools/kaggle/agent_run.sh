#!/bin/bash
# TEMPORARY diagnostics for agent pt01 (removed before merge)
set -u
B=${CARGO_TARGET_DIR:-/tmp/target}/debug
P=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}
D=/tmp/ptdiag; rm -rf $D; mkdir -p $D
C=tools/oracle/cases
echo "== busride backtrace (sumo_core frames)"
RUST_BACKTRACE=1 $B/sumo -n $C/busride/ref_simple.net.xml -r $C/busride/routes.rou $(cat $C/busride/sim.args) \
  --tripinfo-output $D/bus.ti.xml --personinfo-output $D/bus.pi.xml --stop-output $D/bus.stop.xml \
  --no-step-log true > $D/bus.log 2>&1
grep -E 'sumo_core::|crates/sumo_core' $D/bus.log | head -24 | cut -c1-200
for c in city_zurich_tram city_karlsruhe_tram; do
  while read -r v args; do
    [ -n "$v" ] || continue
    o=$D/$c.$v; mkdir -p $o/r
    $B/netconvert --osm-files $C/$c/input.osm --simple-projection ${args//@OUT@/$o/r} -o $o/r/net.net.xml > $o/log 2>&1 </dev/null
    echo "== $c $v rc=$?"
    for f in $C/$c/ref_net_out/$v/*; do
      n=$(basename $f)
      if [ "$n" = net.net.xml ]; then
        python3 tools/oracle/compare_net.py $f $o/r/$n | head -6 | cut -c1-240
      else
        python3 tools/oracle/compare_xml.py $f $o/r/$n | head -3 | cut -c1-240
      fi
    done
  done < $C/$c/net_outputs.txt
done
