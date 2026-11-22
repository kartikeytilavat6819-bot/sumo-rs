#!/bin/bash
# oc97 probe: moped indirect-left rows. Runs official and ours, prints exit codes,
# first stderr lines, arrival differences and the first differing fcd lines of the named vehicle.
OUT=/tmp/oc97
ROOT=$(pwd)
mkdir -p "$OUT"
CASES="tt_junction_model_indirect_left_turn_internal_junction_controlled_moped:flow_0.52 tt_sublane_model_junction_indirect_left_moped:flow_0.15 tt_sublane_model_junction_turnAlignment_moped_indirect_left:flow_0.16"

for c in $CASES; do
  name=${c%%:*}
  veh=${c##*:}
  d="$ROOT/tools/oracle/cases/$name"
  if [ ! -d "$d" ]; then echo "missing case dir $name"; continue; fi
  args=$(sed "s#tools/oracle/cases/$name/##g" "$d/sim.args")
  for side in ref ours; do
    if [ $side = ref ]; then BIN="$SUMO_PRISTINE_BIN/sumo"; else BIN="$CARGO_TARGET_DIR/debug/sumo"; fi
    o="$OUT/$name/$side"
    mkdir -p "$o"
    (cd "$d" && "$BIN" $args --net-file net.net.xml --route-files input_routes.rou.xml \
        --fcd-output "$o/fcd.xml" --tripinfo-output "$o/trip.xml" --no-step-log \
        > "$o/stdout" 2> "$o/stderr"; echo $? > "$o/exit")
  done
  echo "== $name vehicle=$veh exit ref=$(cat "$OUT/$name/ref/exit") ours=$(cat "$OUT/$name/ours/exit")"
  for side in ref ours; do
    echo "-- $side stderr (first 3):"
    head -3 "$OUT/$name/$side/stderr"
  done
  for side in ref ours; do
    echo "-- $side arrival:"
    grep "id=\"$veh\"" "$OUT/$name/$side/trip.xml" | head -1 | cut -c1-200
  done
  echo "-- first fcd difference for $veh (< ref, > ours), first 24 diff lines:"
  diff <(grep "vehicle id=\"$veh\"" "$OUT/$name/ref/fcd.xml") <(grep "vehicle id=\"$veh\"" "$OUT/$name/ours/fcd.xml") | head -24 | cut -c1-200
  # occupants of the internal junction lane at the first differing timestep of the named vehicle
  ext() { awk -v v="$veh" '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} index($0,"vehicle id=\""v"\"")>0{print t}' "$1"; }
  line=$(diff <(grep "vehicle id=\"$veh\"" "$OUT/$name/ref/fcd.xml") <(grep "vehicle id=\"$veh\"" "$OUT/$name/ours/fcd.xml") | head -1 | grep -oE '^[0-9]+')
  if [ -n "$line" ]; then
    T=$(ext "$OUT/$name/ours/fcd.xml" | sed -n "${line}p")
    for side in ref ours; do
      echo "-- $side lanes :C_9_* at t=$T (id lane pos speed):"
      awk -v T="$T" '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} t==T && /lane=":C_9/' "$OUT/$name/$side/fcd.xml" | sed -E 's/.*id="([^"]*)".* speed="([^"]*)" pos="([^"]*)" lane="([^"]*)".*/\1 \4 \3 \2/' | head -12
    done
  fi
done
echo "AGENT RUN DONE"
