#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OFF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
runboth() {
  local c=$1
  local d=tools/oracle/cases/$c
  local extra=""; [ -f $d/sim.args ] && extra=$(cat $d/sim.args)
  echo "===== $c ====="
  $OFF -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.p.ti.xml --fcd-output $OUT/$c.p.fcd.xml --statistic-output $OUT/$c.p.stat.xml --no-step-log true >$OUT/$c.p.log 2>&1
  echo "official exit=$?"
  tail -3 $OUT/$c.p.log
  $OURS -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.o.ti.xml --fcd-output $OUT/$c.o.fcd.xml --statistic-output $OUT/$c.o.stat.xml --no-step-log true >$OUT/$c.o.log 2>&1
  echo "ours exit=$?"
  tail -3 $OUT/$c.o.log
}
for c in tt_bugs_16xxx_ticket16641 tt_bugs_17xxx_ticket17583 tt_bugs_4xxx_ticket4299 tt_basic_routes_parsing_nested_ref tt_basic_routes_parsing_repeat_stops_no_period; do runboth $c; done
echo "=== 16641 tripinfo diff ==="; python tools/oracle/compare_sim.py --tripinfo tools/oracle/cases/tt_bugs_16xxx_ticket16641/ref.tripinfo.xml $OUT/tt_bugs_16xxx_ticket16641.o.ti.xml 2>&1 | head -5
echo "=== 17583 tripinfo diff ==="; python tools/oracle/compare_sim.py --tripinfo tools/oracle/cases/tt_bugs_17xxx_ticket17583/ref.tripinfo.xml $OUT/tt_bugs_17xxx_ticket17583.o.ti.xml 2>&1 | head -5
echo "=== 4299 tripinfo diff ==="; python tools/oracle/compare_sim.py --tripinfo tools/oracle/cases/tt_bugs_4xxx_ticket4299/ref.tripinfo.xml $OUT/tt_bugs_4xxx_ticket4299.o.ti.xml 2>&1 | head -5