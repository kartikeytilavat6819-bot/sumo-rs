#!/bin/bash
# oc104 diagnostics: our sumo vs pristine C++ sumo on the fcd-replay cases.
# sumo's main.rs only runs the default panic hook (which prints file:line) when RUST_BACKTRACE is set.
export RUST_BACKTRACE=full
OURS=${OURS:-/tmp/target/debug/sumo}
OUT=/tmp/oc104_fcd
mkdir -p $OUT
for C in tools/oracle/cases/tt_devices_fcd-replay_plain tools/oracle/cases/tt_devices_fcd-replay_minimal tools/oracle/cases/tt_devices_fcd-replay_persons tools/oracle/cases/tt_devices_fcd-replay_persons_offnet; do
  NAME=$(basename $C)
  EXTRA=$(cat $C/sim.args)
  echo "################ $NAME"
  for side in ours pristine; do
    if [ "$side" = ours ]; then BIN=$OURS; else BIN=$SUMO_PRISTINE_BIN/sumo; fi
    $BIN -n $C/ref_simple.net.xml -r $C/routes.rou --no-step-log true $EXTRA \
      --tripinfo-output $OUT/$NAME.$side.ti.xml --fcd-output $OUT/$NAME.$side.fcd.xml \
      --statistic-output $OUT/$NAME.$side.stat.xml > $OUT/$NAME.$side.log 2>&1
    echo "== $NAME $side exit=$?"
    grep -m3 -E 'panicked at|overflow|Error:' $OUT/$NAME.$side.log | cut -c1-400
    tail -3 $OUT/$NAME.$side.log
  done
  echo "== tripinfo diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.ti.xml $OUT/$NAME.ours.ti.xml | head -60
  echo "== fcd diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.fcd.xml $OUT/$NAME.ours.fcd.xml | head -40
  echo "== stat diff (pristine -> ours)"
  diff -u $OUT/$NAME.pristine.stat.xml $OUT/$NAME.ours.stat.xml | head -40
done
echo "################ PANIC LOCATIONS"
grep -A8 -h -E 'panicked at' $OUT/*.ours.log | grep -E 'panicked at|sumo_core|utils::' | cut -c1-300

# oc105 measurement (from wave2): compare our sumo with the official binary on two target rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc105
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_devices_ssm_bugs_ticket13918 tt_devices_rerouting_reroute_congestions_exit_times; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"
    echo "$side exit=$?"
    echo "$side stderr:"; head -20 "$OUT/$c.$side.err"
    echo "$side stdout:"; head -12 "$OUT/$c.$side.out"
  done
done
