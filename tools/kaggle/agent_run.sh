#!/bin/bash
# rc01 diagnostics (removed before merge): rerouter_parking panic backtrace, containers first diff.
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=${OUT:-/tmp/agent_run}
mkdir -p $OUT
for n in rerouter_parking containers; do
  C=tools/oracle/cases/$n
  RUST_BACKTRACE=1 $OURS -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true \
      --tripinfo-output $OUT/$n.ours.ti.xml $(cat $C/sim.args) > $OUT/$n.ours.log 2>&1
  echo "== $n ours exit=$?"
  grep -m1 -A60 'panicked' $OUT/$n.ours.log | grep -E 'panicked|\.rs:' | cut -c1-200 | head -40
  echo "-- ref truck0/ti:"
  grep -E 'truck0|personinfo' $C/ref.tripinfo.xml | head -4 | cut -c1-250
  echo "-- ours truck0/ti:"
  grep -E 'truck0' $OUT/$n.ours.ti.xml 2>/dev/null | head -4 | cut -c1-250
  python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/$n.ours.ti.xml 2>&1 | head -20
  if [ -n "$SUMO_PRISTINE_BIN" ]; then
    $SUMO_PRISTINE_BIN/sumo -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true \
        --tripinfo-output $OUT/$n.cpp.ti.xml $(cat $C/sim.args) > $OUT/$n.cpp.log 2>&1
    echo "== $n pristine exit=$?"
    python tools/oracle/compare_sim.py --tripinfo $C/ref.tripinfo.xml $OUT/$n.cpp.ti.xml 2>&1 | head -5
  fi
done
