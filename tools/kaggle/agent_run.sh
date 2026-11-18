#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
OURS=${OURS:-$CARGO_TARGET_DIR/debug/sumo}
OFF=${SUMO_PRISTINE_BIN:-/tmp/sumo_pkg/sumo/bin}/sumo
OUT=/tmp/agent_run; rm -rf $OUT; mkdir -p $OUT
for c in tt_basic_routes_parsing_nested_ref tt_basic_routes_parsing_repeat_stops_no_period; do
  d=tools/oracle/cases/$c
  extra=""; [ -f $d/sim.args ] && extra=$(cat $d/sim.args)
  echo "===== $c ====="
  echo "--- official ---"
  $OFF -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.p.ti.xml --fcd-output $OUT/$c.p.fcd.xml --statistic-output $OUT/$c.p.stat.xml --no-step-log true >$OUT/$c.p.log 2>&1
  echo "exit=$?"
  cat $OUT/$c.p.log
  echo "--- ours ---"
  $OURS -n $d/ref_simple.net.xml -r $d/routes.rou $extra --tripinfo-output $OUT/$c.o.ti.xml --fcd-output $OUT/$c.o.fcd.xml --statistic-output $OUT/$c.o.stat.xml --no-step-log true >$OUT/$c.o.log 2>&1
  echo "exit=$?"
  cat $OUT/$c.o.log
done