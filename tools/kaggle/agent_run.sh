#!/usr/bin/env bash
# oc98 probe: run official and our sumo on the target cases, compare tripinfo.
OUT=/tmp/oc98; mkdir -p "$OUT"
OURS="${CARGO_TARGET_DIR:-/tmp/none}/debug/sumo"
OFF="${SUMO_PRISTINE_BIN:-/tmp/none}/sumo"
CASES="tt_rail_rerouter_closing_detour_onInit tt_rail_rerouter_closing_short_closing_wait tt_bugs_17xxx_ticket17583 tt_bugs_4xxx_ticket4299 tt_rail_routing_reverse_on_short_start tt_rail_rail_signal_siding_deadend"
for c in $CASES; do
  d="tools/oracle/cases/$c"
  net=$d/net.net.xml; [ -f "$net" ] || net=$d/ref_simple.net.xml
  rou=$d/routes.rou; for f in input_routes2.rou.xml input_routes.rou.xml; do [ -f "$d/$f" ] && rou=$d/$f && break; done
  args="$(cat "$d/sim.args" 2>/dev/null) -n $net -r $rou"
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    dbg=""; [ "$side" = ours ] && dbg="OC98DBG=1"
    env $dbg timeout 300 "$bin" $args --tripinfo-output "$OUT/$c.$side.tripinfo.xml" --fcd-output "$OUT/$c.$side.fcd.xml" > "$OUT/$c.$side.stdout" 2> "$OUT/$c.$side.stderr"
    echo "== $c $side exit=$?"
    head -n 3 "$OUT/$c.$side.stderr"
  done
  echo "-- first differing tripinfo lines:"
  diff <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.off.tripinfo.xml" 2>/dev/null) <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.ours.tripinfo.xml" 2>/dev/null) 2>&1 | head -n 12
done
# oc98: the 7 expected-error rows. Run exactly like run_all.sh (-n ref_simple.net.xml -r routes.rou + sim.args)
# and compare official vs ours: exit code, stderr, stdout. ref.errors is empty on these rows (TextTest errors.sumo is empty).
for c in tt_rail_rail_signal_siding_deadend tt_rail_routing_reverse_on_short_start tt_rail_rerouter_overtaking_follower_arrival_delay_low tt_rail_rerouter_overtaking_follower_delay_high tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_default tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_high tt_rail_rerouter_overtaking_follower_delay_low; do
  d="tools/oracle/cases/$c"
  a="$(cat "$d/sim.args" 2>/dev/null)"
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $a > "$OUT/$c.e.$side.out" 2> "$OUT/$c.e.$side.err"
    echo $? > "$OUT/$c.e.$side.rc"
  done
  same=SAME
  cmp -s "$OUT/$c.e.off.rc" "$OUT/$c.e.ours.rc" || same=DIFF-rc
  cmp -s "$OUT/$c.e.off.err" "$OUT/$c.e.ours.err" || same="$same DIFF-stderr"
  cmp -s "$OUT/$c.e.off.out" "$OUT/$c.e.ours.out" || same="$same DIFF-stdout"
  echo "== ERRROW $c official_rc=$(cat $OUT/$c.e.off.rc) ours_rc=$(cat $OUT/$c.e.ours.rc) -> $same"
  echo "   official stderr:"; head -n 4 "$OUT/$c.e.off.err" | sed 's/^/     /'
  [ "$same" = SAME ] || { echo "   ours stderr:"; head -n 4 "$OUT/$c.e.ours.err" | sed 's/^/     /'; }
done
# oc98: the REAL overtaking scenario. The imported cases drop the parent's `-a input_additional.add.xml,input_routes.rou.xml`
# (overtakingReroute rerouter + vTypes + routes); SUMO runs it as the union with the child's -a/-r. Compare official vs ours.
PK=tools/kaggle/oc98_overtaking
for c in tt_rail_rerouter_overtaking_follower_arrival_delay_low tt_rail_rerouter_overtaking_follower_delay_high tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_default tt_rail_rerouter_overtaking_follower_delay_high_maxDelay_high tt_rail_rerouter_overtaking_follower_delay_low; do
  d="tools/oracle/cases/$c"
  a="$(sed -E 's/(^| )-a [^ ]+//' "$d/sim.args")"
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    timeout 300 "$bin" $a -n "$d/net.net.xml" -a "$PK/input_additional.add.xml,$PK/input_routes.rou.xml,$d/input_additional2.add.xml" -r "$d/input_routes2.rou.xml"       --no-step-log true --tripinfo-output "$OUT/$c.r.$side.trip.xml" --stop-output "$OUT/$c.r.$side.stop.xml" --vehroute-output "$OUT/$c.r.$side.veh.xml" > "$OUT/$c.r.$side.out" 2> "$OUT/$c.r.$side.err"
    echo $? > "$OUT/$c.r.$side.rc"
  done
  echo "== REAL $c official_rc=$(cat $OUT/$c.r.off.rc) ours_rc=$(cat $OUT/$c.r.ours.rc)  official_trips=$(grep -c '<tripinfo ' $OUT/$c.r.off.trip.xml 2>/dev/null) ours_trips=$(grep -c '<tripinfo ' $OUT/$c.r.ours.trip.xml 2>/dev/null)"
  [ -s "$OUT/$c.r.off.err" ] && { echo "   official stderr:"; head -n 3 "$OUT/$c.r.off.err" | sed 's/^/     /'; }
  [ -s "$OUT/$c.r.ours.err" ] && { echo "   ours stderr:"; head -n 3 "$OUT/$c.r.ours.err" | sed 's/^/     /'; }
  for k in trip stop veh; do
    dd=$(diff <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.r.off.$k.xml" 2>/dev/null) <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.r.ours.$k.xml" 2>/dev/null) | head -n 6 | cut -c1-260)
    if [ -z "$dd" ]; then echo "   $k: SAME"; else echo "   $k: DIFF"; echo "$dd" | sed 's/^/     /'; fi
  done
done
# oc98: newStop_fast_forced(2) complex TraCI rows: official vs ours console (+ OC98DBG addStop lines on our side)
echo "== COMPLEX newStop_fast_forced*"
COMPLEX_OUT="$OUT/complex" COMPLEX_FILTER="setStop/newStop_fast_forced" bash tools/oracle/run_complex_suite.sh 2>&1 | tail -n 4
for t in newStop_fast_forced newStop_fast_forced2; do
  for side in official ours; do
    f="$OUT/complex/vehicle/setStop/$t/$side/console.txt"
    echo "-- $t $side console (first 14 lines):"; grep -a -v "^Retrying\|UserWarning" "$f" 2>/dev/null | head -n 14 | cut -c1-260 | sed 's/^/     /'
  done
done
echo "== 17583 v.25 FCD first differing samples (official vs ours):"
diff <(grep 'id="v.25"' "$OUT/tt_bugs_17xxx_ticket17583.off.fcd.xml" 2>/dev/null) <(grep 'id="v.25"' "$OUT/tt_bugs_17xxx_ticket17583.ours.fcd.xml" 2>/dev/null) | head -n 8
echo "== 17583 ours reroute/origin lines for v.25:"
grep -a "OC98ORIGIN\|reroute veh=v.25" "$OUT/tt_bugs_17xxx_ticket17583.ours.stderr" | head -n 8
echo "AGENT RUN DONE"
