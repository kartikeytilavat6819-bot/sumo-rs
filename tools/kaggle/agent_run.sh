#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OFF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
run() { # case
  local c=$1
  local d=tools/oracle/cases/$c
  local extra=""; [ -f $d/sim.args ] && extra=$(cat $d/sim.args)
  echo "===== $c ====="
  echo "--- ours ---"
  RUST_BACKTRACE=1 $OURS -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.o.ti.xml --fcd-output $OUT/$c.o.fcd.xml --statistic-output $OUT/$c.o.stat.xml --no-step-log true >$OUT/$c.o.log 2>&1
  echo "exit=$?"
  grep -m1 -E 'panicked at|Error:|warning' $OUT/$c.o.log | head -c 300; echo
  tail -8 $OUT/$c.o.log
}
for c in tt_bugs_13xxx_ticket13108 tt_bugs_15xxx_ticket15030 tt_bugs_7xxx_ticket7149 tt_bugs_16xxx_ticket16641 tt_bugs_17xxx_ticket17583 tt_bugs_4xxx_ticket4299 tt_basic_routes_parsing_nested_ref tt_basic_routes_parsing_repeat_stops_no_period; do run $c; done