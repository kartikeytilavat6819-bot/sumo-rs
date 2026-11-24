#!/bin/bash
# oc97 probe: moped indirect-left rows. Runs official and ours, prints exit codes,
# first stderr lines, arrival differences and the first differing fcd lines of the named vehicle.
OUT=/tmp/oc97
ROOT=$(pwd)
mkdir -p "$OUT"
rm -f "$OUT/lc_trace.txt"
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
  # trajectory of flow_0.15 (blocks flow_0.16 on SC_0 in turnAlignment): first 12 fcd lines per side
  echo "-- flow_0.15 fcd diff (< ref, > ours), first 30 diff lines:"
  diff <(grep 'vehicle id="flow_0.15"' "$OUT/$name/ref/fcd.xml") <(grep 'vehicle id="flow_0.15"' "$OUT/$name/ours/fcd.xml") | head -30 | cut -c1-160
  # occupants of the internal junction lane at the first differing timestep of the named vehicle
  ext() { awk -v v="$veh" '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} index($0,"vehicle id=\""v"\"")>0{print t}' "$1"; }
  line=$(diff <(grep "vehicle id=\"$veh\"" "$OUT/$name/ref/fcd.xml") <(grep "vehicle id=\"$veh\"" "$OUT/$name/ours/fcd.xml") | head -1 | grep -oE '^[0-9]+')
  if [ -n "$line" ]; then
    T=$(ext "$OUT/$name/ours/fcd.xml" | sed -n "${line}p")
    for side in ref ours; do
      echo "-- $side all vehicles at t=$T (id lane pos speed):"
      awk -v T="$T" '/<timestep/{match($0,/time="[^"]*"/); t=substr($0,RSTART+6,RLENGTH-7)} t==T && /<vehicle/' "$OUT/$name/$side/fcd.xml" | sed -E 's/.*id="([^"]*)".* speed="([^"]*)" pos="([^"]*)" lane="([^"]*)".*/\1 \4 \3 \2/' | head -20
    done
  fi
done
echo "-- lane-change trace (ours, temp), lines: $(wc -l < "$OUT/lc_trace.txt" 2>/dev/null)"
grep "speed=0.000000 " "$OUT/lc_trace.txt" 2>/dev/null | grep -v "off=0 " | head -20
# oc97 rail rerouter rows (new assignment): run both binaries, print first stderr/tripinfo differences
RAIL="tt_rail_rerouter_overtaking_follower_arrival_delay_low tt_rail_rerouter_overtaking_follower_delay_high tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_default tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_high tt_rail_rerouter_overtaking_follower_delay_low tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait tt_rail_rail_signal_moving_block_protect_bidi"
for name in $RAIL; do
  d="$ROOT/tools/oracle/cases/$name"
  if [ ! -d "$d" ]; then echo "missing $name"; continue; fi
  args=$(sed "s#tools/oracle/cases/$name/##g" "$d/sim.args")
  for side in ref ours; do
    if [ $side = ref ]; then BIN="$SUMO_PRISTINE_BIN/sumo"; else BIN="$CARGO_TARGET_DIR/debug/sumo"; fi
    o="$OUT/rail/$name/$side"
    mkdir -p "$o"
    (cd "$d" && "$BIN" $args --net-file net.net.xml --route-files input_routes.rou.xml --tripinfo-output "$o/trip.xml" --fcd-output "$o/fcd.xml" --vehroute-output "$o/vehroute.xml" --vehroute-output.exit-times --no-step-log > "$o/stdout" 2> "$o/stderr"; echo $? > "$o/exit")
  done
  echo "== $name exit ref=$(cat "$OUT/rail/$name/ref/exit") ours=$(cat "$OUT/rail/$name/ours/exit")"
  diff "$OUT/rail/$name/ref/stderr" "$OUT/rail/$name/ours/stderr" | head -4 | cut -c1-200
  diff "$OUT/rail/$name/ref/trip.xml" "$OUT/rail/$name/ours/trip.xml" | head -6 | cut -c1-200
  for side in ref ours; do echo "-- $side vehroute (v_0):"; grep -A3 "vehicle id=\"v_0\"" "$OUT/rail/$name/$side/vehroute.xml" 2>/dev/null | head -4 | cut -c1-300; done
  nn=4; [ "$name" = tt_rail_rail_signal_moving_block_protect_bidi ] && nn=40; for veh in WE1 WE0 EW v_0; do diff <(grep "vehicle id=\"$veh\"" "$OUT/rail/$name/ref/fcd.xml") <(grep "vehicle id=\"$veh\"" "$OUT/rail/$name/ours/fcd.xml") | head -$nn | cut -c1-170; done
done
# oc97: route costs for the train on the closing case, official vs ours (TraCI findRoute)
for side in ref ours; do
  if [ $side = ref ]; then BIN="$SUMO_PRISTINE_BIN/sumo"; else BIN="$CARGO_TARGET_DIR/debug/sumo"; fi
  echo "-- findRoute $side:"
  python3 "$ROOT/tools/kaggle/oc97_route_probe.py" "$BIN" "$ROOT/tools/oracle/cases/tt_rail_rerouter_closing_detour_onInit" 2>&1 | head -8 | cut -c1-300
done
echo "AGENT RUN DONE"
