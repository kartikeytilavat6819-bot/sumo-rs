#!/bin/bash
# oc101 probe: where do the two ticket9812 rail-taxi cases spin in our binary,
# and what error does the official sumo emit? Runs on the Linux gate VM.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
OUT=/tmp/oc101-probe
mkdir -p "$OUT"
CASES="tt_devices_taxi_bugs_ticket9812_railRouter tt_devices_taxi_bugs_ticket9812_taxi_vclasses"

for c in $CASES; do
  d="tools/oracle/cases/$c"
  EXTRA=$(cat "$d/sim.args")
  echo "===== $c : OURS (gdb, 25s SIGINT) ====="
  timeout -s INT 25 gdb -q -batch \
    -ex run -ex 'thread apply all bt 50' \
    --args "$BIN/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA \
    >"$OUT/$c.gdb" 2>&1
  grep -m 1 -E 'Program received signal|SIGINT' "$OUT/$c.gdb" || true
  tail -90 "$OUT/$c.gdb"
  echo "===== $c : OFFICIAL (error text) ====="
  if [ -x "$SUMO_PRISTINE_BIN/sumo" ]; then
    timeout 60 "$SUMO_PRISTINE_BIN/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/$c.off.ti.xml" --fcd-output "$OUT/$c.off.fcd.xml" \
      --statistic-output "$OUT/$c.off.stat.xml" --no-step-log true $EXTRA \
      >"$OUT/$c.off.log" 2>&1
    echo "rc=$?"
    grep -m 5 -E 'Error|Warning|Quitting' "$OUT/$c.off.log" || true
  else
    echo "no official binary at $SUMO_PRISTINE_BIN/sumo"
  fi
done
