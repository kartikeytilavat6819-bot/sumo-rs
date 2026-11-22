#!/bin/bash
# oc101 probe v2: locate the ticket9812 rail-taxi spin precisely.
# Runs on the Linux gate VM; writes only under /tmp.
set -u
cd "$(git rev-parse --show-toplevel)" || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
OUT=/tmp/oc101-probe
mkdir -p "$OUT"
CASES="tt_devices_taxi_bugs_ticket9812_railRouter tt_devices_taxi_bugs_ticket9812_taxi_vclasses"

for c in $CASES; do
  d="tools/oracle/cases/$c"
  EXTRA=$(cat "$d/sim.args")
  for secs in 8 20; do
    echo "===== $c : OURS (gdb, ${secs}s SIGINT) ====="
    timeout -s INT "$secs" gdb -q -batch \
      -ex 'set debuginfod enabled off' -ex 'set pagination off' \
      -ex run -ex 'bt 60' -ex 'info symbol $pc' -ex 'x/24i $pc-48' \
      --args "$BIN/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
        --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
        --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA \
      >"$OUT/$c.$secs.gdb" 2>&1
    grep -m 1 -E 'Program received signal|SIGINT' "$OUT/$c.$secs.gdb" || true
    grep -m 1 -E '^0x[0-9a-f]+ in' "$OUT/$c.$secs.gdb" || true
    grep -m 1 'No connection between stop' "$OUT/$c.$secs.gdb" || true
    # print the first 25 frames and the disassembly block
    sed -n '/^#0 /,/^#25 /p' "$OUT/$c.$secs.gdb" | head -30
    grep -A 26 -m 1 '^   0x.*:' "$OUT/$c.$secs.gdb" | head -28
  done
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
