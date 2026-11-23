#!/bin/bash
# oc119 probe: runs official ($SUMO_PRISTINE_BIN/sumo) and ours on each case; prints first differences of
# tripinfo/fcd/statistics, and of stdout/stderr against the committed ref.output / ref.errors.
# Usage (repo root): bash tools/kaggle/agent_run.sh [case ...]
OUT=/tmp/oc119; mkdir -p "$OUT"
ROOT=$(pwd)
OURS="${CARGO_TARGET_DIR:-$ROOT/target}/debug/sumo"
OFF="${SUMO_PRISTINE_BIN:-/usr/local/bin}/sumo"
CASES=("$@")
[ ${#CASES[@]} -eq 0 ] && CASES=(tt_output_tripinfo_one_vehicle_subsecond tt_output_tripinfo_probability_by_type tt_output_tripinfo_teleport tt_output_tripinfo_teleport_multihop tt_output_tripinfo_write_undeparted)
F="grep -v -e generated -e -output"
for c in "${CASES[@]}"; do
  dir="$ROOT/tools/oracle/cases/$c"
  for tag in off ours; do
    bin=$OFF; [ $tag = ours ] && bin=$OURS
    w="$OUT/$c.$tag"; mkdir -p "$w"
    args=$(tr -d '\r' < "$dir/sim.args" | sed "s#tools/oracle/cases/$c#$dir#g")
    (cd "$dir" && timeout 600 "$bin" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$w/ti.xml" \
        --fcd-output "$w/fcd.xml" --statistic-output "$w/stat.xml" --no-step-log true $args \
        > "$w/out" 2> "$w/err"); echo "$c $tag rc=$?"
  done
  for k in ti fcd stat; do
    if [ -f "$OUT/$c.off/$k.xml" ] && [ -f "$OUT/$c.ours/$k.xml" ]; then
      n=$(diff <($F < "$OUT/$c.off/$k.xml") <($F < "$OUT/$c.ours/$k.xml") | grep -c "^[<>]")
      echo "$c $k difflines=$n"
    else
      echo "$c $k missing output"
    fi
  done
  for tag in off ours; do
    for s in out err; do
      ref=ref.output; [ $s = err ] && ref=ref.errors
      if [ -f "$dir/$ref" ]; then
        d=$(diff <(tr -d '\r' < "$dir/$ref") "$OUT/$c.$tag/$s" | head -6 | cut -c1-200)
        echo "$c $tag $s vs $ref: ${d:-SAME}"
        echo "$c $tag $s tail:"; tail -n 3 "$OUT/$c.$tag/$s" | cut -c1-200
      fi
    done
  done
done
echo "AGENT RUN DONE"
