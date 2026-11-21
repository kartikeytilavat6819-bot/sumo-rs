#!/bin/bash
# oc93 measurement: for a few rail deadlock cases, report how far each binary
# got and whether it teleported due to a rail-signal deadlock.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"

OUR="$CARGO_TARGET_DIR/debug/sumo"
CPP="$SUMO_PRISTINE_BIN/sumo"

OUT=${OC93_OUT:-/tmp/oc93}
mkdir -p "$OUT"

CASES="tt_rail_siding_loop_3veh tt_rail_siding_siding_deadlock tt_rail_collision_frontal_collision tt_rail_bugs_double_stop"

run_one() {
  label=$1; bin=$2; c=$3
  dir="tools/oracle/cases/$c"
  args=$(cat "$dir/sim.args" 2>/dev/null | tr -d '\r')
  log="$OUT/$c.$label.log"
  fcd="$OUT/$c.$label.fcd.xml"
  rm -f "$log" "$fcd"
  timeout 60 "$bin" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --fcd-output "$fcd" --no-step-log true \
      $args > "$log" 2>&1
  rc=$?
  last=$(grep -a '<timestep' "$fcd" 2>/dev/null | tail -1 | grep -ao 'time="[0-9.]*"' | tail -1)
  tele=$(grep -ac 'Teleporting vehicle' "$log" 2>/dev/null)
  rstele=$(grep -ac 'railSignal' "$log" 2>/dev/null)
  dl=$(grep -a -m1 -E 'deadlock|Deadlock' "$log" 2>/dev/null | cut -c1-100)
  lastline=$(grep -a -v '^$' "$log" 2>/dev/null | tail -1 | cut -c1-100)
  printf '%-4s %-42s rc=%-3s last=%-12s tele=%-3s rsTele=%-3s warn=%s\n' \
      "$label" "$c" "$rc" "${last:-none}" "${tele:-0}" "${rstele:-0}" "${dl:-}"
  printf '       lastline: %s\n' "${lastline:-}"
}

for c in $CASES; do
  echo "===== $c ====="
  run_one CPP "$CPP" "$c"
  run_one OUR "$OUR" "$c"
done
