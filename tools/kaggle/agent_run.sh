#!/bin/bash
# oc93 probe 4: pinpoint the exact source line of the infinite loop inside
# SUMOAbstractRouter::prohibit via gdb's `info line *$pc`.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUR="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc93
mkdir -p "$OUT"

for c in tt_rail_siding_loop_3veh tt_rail_collision_frontal_collision; do
  dir="tools/oracle/cases/$c"
  args=$(cat "$dir/sim.args" | tr -d '\r')
  dst="$OUT/pc_$c.txt"
  timeout -s INT 22 gdb -q -batch \
      -ex 'set pagination off' -ex 'set confirm off' \
      -ex 'run' \
      -ex 'printf "\n==== PC INFO ====\n"' \
      -ex 'info symbol $pc' \
      -ex 'info line *$pc' \
      -ex 'frame 0' -ex 'info frame' \
      -ex 'x/6i $pc' \
      -ex 'printf "\n==== BACKTRACE ====\n"' \
      -ex 'bt 25' \
      -ex 'kill' \
      --args "$OUR" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --fcd-output "$OUT/$c.pc.fcd.xml" --no-step-log true $args \
      >"$dst" 2>&1
  echo "========== $c =========="
  sed -n '/==== PC INFO ====/,/==== BACKTRACE ====/p' "$dst" | head -50
  echo "---- backtrace ----"
  sed -n '/==== BACKTRACE ====/,$p' "$dst" | grep -aE '^#[0-9]+' | head -25
  echo "---- last fcd: $(grep -a '<timestep' "$OUT/$c.pc.fcd.xml" 2>/dev/null | tail -1 | grep -ao 'time="[0-9.]*"' | tail -1)"
done
