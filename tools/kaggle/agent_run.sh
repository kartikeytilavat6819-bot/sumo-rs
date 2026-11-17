#!/bin/bash
# MEASUREMENT ONLY (cf01): first FCD / tripinfo difference, ours vs official C++, for the cfmodels FAIL variants.
# Remove before merging to main.
set -u
OURS=$CARGO_TARGET_DIR/debug/sumo
CPP=$SUMO_PRISTINE_BIN/sumo
C=tools/oracle/cases/cfmodels
D=/tmp/cf_meas
mkdir -p $D
for v in ${CF_VARIANTS:-KraussPS CACC W99_cc1}; do
  flags=$(cat $C/sim_$v.args)
  for side in ours cpp; do
    bin=$OURS; [ $side = cpp ] && bin=$CPP
    $bin -n $C/ref_simple.net.xml -r $C/routes.rou $flags \
      --fcd-output $D/$v.$side.fcd.xml --tripinfo-output $D/$v.$side.trip.xml \
      --no-step-log true --no-warnings true > $D/$v.$side.log 2>&1
    echo "$v $side exit $?"
  done
  echo "== $v fcd (ref=cpp cand=ours):"
  python3 tools/oracle/compare_sim.py --fcd $D/$v.cpp.fcd.xml $D/$v.ours.fcd.xml
  echo "== $v tripinfo:"
  python3 tools/oracle/compare_sim.py --tripinfo $D/$v.cpp.trip.xml $D/$v.ours.trip.xml
done
