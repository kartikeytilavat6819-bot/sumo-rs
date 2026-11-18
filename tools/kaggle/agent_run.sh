#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
for c in tt_bugs_13xxx_ticket13108 tt_bugs_15xxx_ticket15030 tt_bugs_7xxx_ticket7149 tt_bugs_16xxx_ticket16641 tt_bugs_17xxx_ticket17583 tt_bugs_4xxx_ticket4299; do
  d=tools/oracle/cases/$c
  extra=""; [ -f $d/sim.args ] && extra=$(cat $d/sim.args)
  $OURS -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.ti.xml --fcd-output $OUT/$c.fcd.xml --statistic-output $OUT/$c.stat.xml --no-step-log true >$OUT/$c.log 2>&1
  echo "$c exit=$? $(grep -m1 'panicked' $OUT/$c.log)"
  python tools/oracle/compare_sim.py --tripinfo $d/ref.tripinfo.xml $OUT/$c.ti.xml 2>&1 | head -1
done