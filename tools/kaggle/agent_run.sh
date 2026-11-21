#!/bin/bash
# oc93 probe 2: our binary hangs inside a simulation step. Attach a stack
# twice, 5s apart, to the stuck process, then show the last fcd timesteps.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUR="$CARGO_TARGET_DIR/debug/sumo"
OUT=/tmp/oc93
mkdir -p "$OUT"

echo "gdb: $(command -v gdb 2>/dev/null || echo none)  eu-stack: $(command -v eu-stack 2>/dev/null || echo none)"

stack_of() {
  pid=$1; dst=$2
  if command -v gdb >/dev/null 2>&1; then
    gdb -p "$pid" -batch -ex 'thread apply all bt 40' >"$dst" 2>&1
    if grep -qi 'ptrace: Operation not permitted' "$dst"; then :; fi
  fi
  if [ ! -s "$dst" ] || grep -qi 'ptrace: Operation not permitted\|No such process' "$dst"; then
    if command -v eu-stack >/dev/null 2>&1; then
      eu-stack -p "$pid" >"$dst" 2>&1
    else
      cat "/proc/$pid/stack" >"$dst" 2>&1
    fi
  fi
}

for c in tt_rail_siding_loop_3veh tt_rail_collision_frontal_collision; do
  dir="tools/oracle/cases/$c"
  args=$(cat "$dir/sim.args" | tr -d '\r')
  fcd="$OUT/$c.fcd.xml"
  log="$OUT/$c.log"
  rm -f "$fcd" "$log"
  "$OUR" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --fcd-output "$fcd" --no-step-log true $args >"$log" 2>&1 &
  pid=$!
  sleep 15
  alive=no; kill -0 "$pid" 2>/dev/null && alive=yes
  echo "===== $c pid=$pid alive=$alive ====="
  s1="$OUT/$c.stack1.txt"; s2="$OUT/$c.stack2.txt"
  stack_of "$pid" "$s1"
  echo "--- stack 1 ---"
  head -70 "$s1"
  sleep 5
  stack_of "$pid" "$s2"
  echo "--- stack 2 ---"
  head -70 "$s2"
  echo "--- frames common to both (loop signature) ---"
  grep -aoE '=>? ?\(.*\)|[A-Za-z_][A-Za-z0-9_:]*::[A-Za-z0-9_]+' "$s1" 2>/dev/null | sort | uniq -c | sort -rn | head -12
  kill -9 "$pid" 2>/dev/null
  wait "$pid" 2>/dev/null
  echo "--- last fcd timesteps + vehicles ---"
  grep -aE '<timestep|<vehicle' "$fcd" 2>/dev/null | tail -40
done
