#!/bin/bash
# oc119 probe: runs official ($SUMO_PRISTINE_BIN/sumo) and ours on each target case and prints the first differences.
# Usage (repo root): bash tools/kaggle/agent_run.sh [case ...]   (default: the oc119 target rows)
OUT=/tmp/oc119; mkdir -p "$OUT"
ROOT=$(pwd)
OURS="${CARGO_TARGET_DIR:-$ROOT/target}/debug/sumo"
OFF="${SUMO_PRISTINE_BIN:-/usr/local/bin}/sumo"
CASES=("$@")
[ ${#CASES[@]} -eq 0 ] && CASES=(tt_devices_rerouting_tlsPenalty_penalty_0.5 tt_devices_stationfinder_charging tt_devices_fcd-replay_plain tt_output_examples_cross3ltl_suffix tt_devices_taxi_idling_randomCircling_rerouting)
for c in "${CASES[@]}"; do
  dir="$ROOT/tools/oracle/cases/$c"
  for tag in off ours; do
    bin=$OFF; [ $tag = ours ] && bin=$OURS
    w="$OUT/$c.$tag"; mkdir -p "$w"
    args=$(tr -d '\r' < "$dir/sim.args" | sed "s#tools/oracle/cases/$c#$dir#g")
    (cd "$dir" && timeout 600 "$bin" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$w/ti.xml" \
        --fcd-output "$w/fcd.xml" --statistic-output "$w/stat.xml" --no-step-log true $args \
        > "$w/log" 2>&1); echo "$c $tag rc=$?"
  done
  for k in ti fcd stat; do
    if [ -f "$OUT/$c.off/$k.xml" ] && [ -f "$OUT/$c.ours/$k.xml" ]; then
      F="grep -v -e generated -e -output"; n=$(diff <($F < "$OUT/$c.off/$k.xml") <($F < "$OUT/$c.ours/$k.xml") | grep -c "^[<>]")
      echo "$c $k difflines=$n"
      diff -U2 <($F < "$OUT/$c.off/$k.xml") <($F < "$OUT/$c.ours/$k.xml") | head -12 | cut -c1-200
    else
      echo "$c $k missing output"
    fi
  done
  echo "--- ours log tail:"; tail -n 3 "$OUT/$c.ours/log" | cut -c1-200
done
