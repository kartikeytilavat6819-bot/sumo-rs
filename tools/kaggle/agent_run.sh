#!/bin/bash
# oc93 probe 3: run our sumo UNDER gdb (ptrace_scope=1 -> only descendants),
# interrupt inside the stuck step, and print the backtrace twice.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUR="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc93
mkdir -p "$OUT"
echo "gdb: $(command -v gdb 2>/dev/null || echo none)"

gdb_bt() {
  c=$1; secs=$2; tag=$3
  dir="tools/oracle/cases/$c"
  args=$(cat "$dir/sim.args" | tr -d '\r')
  dst="$OUT/bt_$c.$tag.txt"
  timeout -s INT "$secs" gdb -q -batch \
      -ex 'set pagination off' -ex 'set confirm off' \
      -ex 'run' -ex 'thread apply all bt 50' -ex 'kill' \
      --args "$OUR" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --fcd-output "$OUT/$c.$tag.fcd.xml" --no-step-log true $args \
      >"$dst" 2>&1
  echo "===== $c bt $tag (${secs}s) ====="
  grep -aE '^#[0-9]+|^Thread|SIGINT|Program received' "$dst" | head -34
  echo "      last fcd time: $(grep -a '<timestep' "$OUT/$c.$tag.fcd.xml" 2>/dev/null | tail -1 | grep -ao 'time="[0-9.]*"' | tail -1)"
}

for c in tt_rail_siding_loop_3veh tt_rail_collision_frontal_collision; do
  gdb_bt "$c" 20 a
  gdb_bt "$c" 35 b
done
