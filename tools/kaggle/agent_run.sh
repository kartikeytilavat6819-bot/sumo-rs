#!/bin/bash
# sc03 measurement: backtraces of the ptr.rs:77 null deref and first diffs of the failing rows.
# TEMPORARY: removed before merge.
set -u
ROOT=$(pwd)
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
OUT=/tmp/sc03
mkdir -p "$OUT"
export RUST_BACKTRACE=1

run_case() {
  local c=$1 tag=$2 dir=$ROOT/tools/oracle/cases/$1
  local EXTRA=""
  if [ "$tag" = ref ]; then
    [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
  else
    EXTRA=$(cat "$dir/sim_${tag#ref_}.args" 2>/dev/null)
  fi
  echo "=== $c/$tag"
  "$BIN" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
    --tripinfo-output "$OUT/$c.$tag.ti.xml" --fcd-output "$OUT/$c.$tag.fcd.xml" \
    --statistic-output "$OUT/$c.$tag.stat.xml" --no-step-log true $EXTRA \
    >"$OUT/$c.$tag.sumo.log" 2>&1
  local rc=$?
  echo "rc=$rc"
  if grep -q panicked "$OUT/$c.$tag.sumo.log"; then
    grep -m1 panicked "$OUT/$c.$tag.sumo.log" | cut -c1-200
    grep -A30 panicked "$OUT/$c.$tag.sumo.log" | grep -E 'sumo_core|utils' | head -12
  else
    python tools/oracle/compare_sim.py --tripinfo "$dir/$tag.tripinfo.xml" "$OUT/$c.$tag.ti.xml" 2>&1 | head -3
    if [ -f "$dir/$tag.fcd.xml" ]; then
      python tools/oracle/compare_sim.py --fcd "$dir/$tag.fcd.xml" "$OUT/$c.$tag.fcd.xml" 2>&1 | head -3
    fi
  fi
}

run_case jt_traffic_light_unregulated ref
grep -E 'NULLJ|NLPI' "$OUT/jt_traffic_light_unregulated.ref.sumo.log" | head -24
run_case city_plano ref
run_case city_bengaluru_blr ref

echo "=== depart variants (failing rows only)"
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | grep -v ' PASS '
