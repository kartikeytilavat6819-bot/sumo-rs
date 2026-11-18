#!/bin/bash
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}
BIN="$CARGO_TARGET_DIR/debug"
CASE=${CASE:-tt_output_personinfo_pedestrians}
D="tools/oracle/cases/$CASE"
OUT="/tmp/agent_run_out"; rm -rf "$OUT"; mkdir -p "$OUT"
EXTRA=""
[ -f "$D/sim.args" ] && EXTRA=$(cat "$D/sim.args")
"$BIN/sumo.exe" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" \
  --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
  --statistic-output "$OUT/stat.xml" --no-step-log true $EXTRA \
  >"$OUT/run.log" 2>&1
rc=$?
echo "exit=$rc"
if [ $rc -ne 0 ]; then head -20 "$OUT/run.log"; exit 0; fi
for kind in ti fcd stat; do
  ref="$D/ref.$kind.xml"; [ "$kind" = ti ] && ref="$D/ref.tripinfo.xml"
  [ -f "$ref" ] || continue
  cand="$OUT/$kind.xml"; [ "$kind" = ti ] && cand="$OUT/ti.xml"; [ "$kind" = fcd ] && cand="$OUT/fcd.xml"; [ "$kind" = stat ] && cand="$OUT/stat.xml"
  if [ "$kind" = fcd ]; then
    python tools/oracle/compare_sim.py --fcd "$ref" "$cand" >"$OUT/$kind.log" 2>&1
  elif [ "$kind" = stat ]; then
    python tools/oracle/compare_stat.py "$ref" "$cand" >"$OUT/$kind.log" 2>&1
  else
    python tools/oracle/compare_sim.py --tripinfo "$ref" "$cand" >"$OUT/$kind.log" 2>&1
  fi
  echo "=== $kind ==="; head -20 "$OUT/$kind.log"
done