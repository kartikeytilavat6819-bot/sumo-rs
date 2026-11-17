#!/bin/bash
# Rail case diagnostics: our sumo and the pristine C++ sumo ($SUMO_PRISTINE_BIN/sumo) on the same net and routes.
# Prints per side: departs/delays from tripinfo, and trainEast FCD speed for t=0..40.
# Run from the repo root with OURS (default /tmp/target/debug/sumo) and SUMO_PRISTINE_BIN set.
C=tools/oracle/cases/rail
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p $OUT
ARGS="-n $C/ref_simple.net.xml -r $C/routes.rou --end 60 --time-to-teleport=-1 --no-step-log true"
for side in ours pristine; do
  if [ $side = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
  $BIN $ARGS --tripinfo-output $OUT/$side.ti.xml --fcd-output $OUT/$side.fcd.xml > $OUT/$side.log 2>&1
  echo "== $side exit=$?"
  python3 $C/agent_summary.py $OUT/$side.ti.xml $OUT/$side.fcd.xml | tee -a $OUT/summary.txt
done
