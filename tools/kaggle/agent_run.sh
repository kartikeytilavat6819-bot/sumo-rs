#!/bin/bash
# Combined measurement script (oc101 probe + oc105 expected-error rows).
# Runs on the Linux gate VM from the repo root; writes only under /tmp.
set -u
cd "$(dirname "$0")/../.." || exit 1

# ---- oc101: locate the ticket9812 rail-taxi spin precisely ----
OUT=/tmp/oc101-probe
mkdir -p "$OUT"
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
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

# oc101: numeric divergence in idling_randomCircling_rerouting
c=tt_devices_taxi_idling_randomCircling_rerouting
d="tools/oracle/cases/$c"
EXTRA=$(cat "$d/sim.args")
echo "===== $c : OURS vs REF (first diffs) ====="
timeout 120 "$BIN/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
  --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
  --statistic-output "$OUT/$c.stat.xml" --no-step-log true $EXTRA \
  >"$OUT/$c.log" 2>&1
echo "rc=$?"
python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -20
python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | head -20

# ---- oc105: compare our sumo with the official binary on two target rows ----
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
