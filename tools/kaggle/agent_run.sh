#!/bin/bash
# Measurement only (mc02): meso cases through our sumo, compared with pristine refs. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
BIN=/tmp/target/debug
for c in meso_junction meso_real meso_city_plano; do
  d=tools/oracle/cases/$c
  out=/tmp/mo/$c; mkdir -p $out
  flags=$(cat $d/sim.args)
  $BIN/sumo -n $d/ref_simple.net.xml -r $d/routes.rou --tripinfo-output $out/ti.xml \
      --fcd-output $out/fcd.xml --statistic-output $out/stat.xml --no-step-log true $flags > $out/log 2>&1
  rc=$?
  echo "== $c rc=$rc $(grep -m1 -E 'panicked|rror' $out/log | cut -c1-160)"
  python3 tools/oracle/compare_sim.py --tripinfo $d/ref.tripinfo.xml $out/ti.xml 2>&1 | head -3
  python3 tools/oracle/compare_sim.py --fcd $d/ref.fcd.xml $out/fcd.xml 2>&1 | head -3
  python3 tools/oracle/compare_stat.py $d/ref.stat.xml $out/stat.xml 2>&1 | tail -2
done
d=tools/oracle/cases/meso_junction
echo "--- junction tripinfo ref"; grep -m4 '<tripinfo ' $d/ref.tripinfo.xml | cut -c1-260
echo "--- junction tripinfo ours"; grep -m4 '<tripinfo ' /tmp/mo/meso_junction/ti.xml | cut -c1-260
echo "--- junction fcd ref t<=1"; awk '/<timestep/{p=1} p&&n<12{print substr($0,1,200); n++} /<\/timestep>/{if(n>=12)exit}' $d/ref.fcd.xml | head -12
echo "--- junction fcd ours t<=1"; awk '/<timestep/{p=1} p&&n<12{print substr($0,1,200); n++} /<\/timestep>/{if(n>=12)exit}' /tmp/mo/meso_junction/fcd.xml | head -12
echo "--- junction stat ref"; grep -E 'vehicleTripStatistics|vehicles' $d/ref.stat.xml | head -4 | cut -c1-200
echo "--- junction stat ours"; grep -E 'vehicleTripStatistics|vehicles' /tmp/mo/meso_junction/stat.xml | head -4 | cut -c1-200
