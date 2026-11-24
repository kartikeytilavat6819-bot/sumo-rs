#!/usr/bin/env bash
# oc98 probe (compact): official $SUMO_PRISTINE_BIN vs ours on the assigned rows.
OUT=/tmp/oc98; mkdir -p "$OUT"
OURS="${CARGO_TARGET_DIR:-/tmp/none}/debug/sumo"
OFF="${SUMO_PRISTINE_BIN:-/tmp/none}/sumo"
cut200() { cut -c1-200; }

# 1) tt_bugs_4xxx_ticket4299: ES1 trajectory, first differing samples with context ('<' official, '>' ours)
c=tt_bugs_4xxx_ticket4299; d=tools/oracle/cases/$c
for side in off ours; do
  bin=$OFF; [ "$side" = ours ] && bin=$OURS
  timeout 300 "$bin" $(cat "$d/sim.args") -n "$d/net.net.xml" -r "$d/input_routes.rou.xml" --fcd-output "$OUT/$c.$side.fcd.xml" > /dev/null 2> "$OUT/$c.$side.err"
done
ES1='/<timestep/{match($0,/time="[^"]*"/);t=substr($0,RSTART+6,RLENGTH-7)} /id="ES1"/{match($0,/speed="[^"]*"/);s=substr($0,RSTART+7,RLENGTH-8);match($0,/pos="[^"]*"/);p=substr($0,RSTART+5,RLENGTH-6);match($0,/lane="[^"]*"/);l=substr($0,RSTART+6,RLENGTH-7);print t" "s" "p" "l}'
paste -d'|' <(awk "$ES1" "$OUT/$c.off.fcd.xml") <(awk "$ES1" "$OUT/$c.ours.fcd.xml") > "$OUT/$c.es1.cmp"
echo "== 4299 ES1 'time speed pos lane' official | ours (3 lines before the first difference, then the first 14 differing):"
awk -F'|' '$1!=$2{print NR; exit}' "$OUT/$c.es1.cmp" > "$OUT/$c.es1.first"
f=$(cat "$OUT/$c.es1.first"); [ -n "$f" ] && sed -n "$((f>3?f-3:1)),$((f+13))p" "$OUT/$c.es1.cmp" | cut200
T=$(sed -n "${f}p" "$OUT/$c.es1.cmp" | cut -d' ' -f1)
echo "== 4299 all vehicles at time $T (official then ours: id lane pos speed):"
for side in off ours; do
  awk -v T="$T" '/<timestep/{match($0,/time="[^"]*"/);cur=substr($0,RSTART+6,RLENGTH-7)} cur==T && /<vehicle /{match($0,/id="[^"]*"/);i=substr($0,RSTART+4,RLENGTH-5);match($0,/lane="[^"]*"/);l=substr($0,RSTART+6,RLENGTH-7);match($0,/pos="[^"]*"/);p=substr($0,RSTART+5,RLENGTH-6);match($0,/speed="[^"]*"/);s=substr($0,RSTART+7,RLENGTH-8);print "   "i" "l" "p" "s}' "$OUT/$c.$side.fcd.xml" | head -n 8
  echo "   --"
done
echo "== 4299 stderr official / ours:"; head -n 3 "$OUT/$c.off.err" | cut200; echo "  --"; head -n 5 "$OUT/$c.ours.err" | cut200

# 2) assigned expected-error rows, run exactly like run_all.sh (-n ref_simple.net.xml -r routes.rou + sim.args)
for c in tt_spec_flow_errors_poisson_invalid tt_spec_arrival_errors_lane_given10 tt_spec_arrival_errors_speed_given100 tt_devices_ssm_bugs_ticket5231; do
  d=tools/oracle/cases/$c; a="$(cat "$d/sim.args" 2>/dev/null)"
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $a > "$OUT/$c.e.$side.out" 2> "$OUT/$c.e.$side.err"
    echo $? > "$OUT/$c.e.$side.rc"
  done
  echo "== ERRROW $c rc official=$(cat $OUT/$c.e.off.rc) ours=$(cat $OUT/$c.e.ours.rc)"
  for k in err out; do
    if cmp -s "$OUT/$c.e.off.$k" "$OUT/$c.e.ours.$k"; then echo "   std$k: SAME ($(wc -l < "$OUT/$c.e.off.$k") lines)"; else
      echo "   std$k DIFF  official:"; sed -E 's/[(][0-9]+ms[)]/(Nms)/' "$OUT/$c.e.off.$k" | head -n 8 | cut200 | sed 's/^/      /'
      echo "              ours:";     sed -E 's/[(][0-9]+ms[)]/(Nms)/' "$OUT/$c.e.ours.$k" | head -n 8 | cut200 | sed 's/^/      /'
    fi
  done
  echo "   ref.errors:"; head -n 3 "$d/ref.errors" 2>/dev/null | cut200 | sed 's/^/      /'
  echo "   ref.output ($(wc -c < "$d/ref.output" 2>/dev/null) bytes):"; sed -E 's/[(][0-9]+ms[)]/(Nms)/' "$d/ref.output" 2>/dev/null | head -n 8 | cut200 | sed 's/^/      /'
done

# 3) tt_bugs_17xxx_ticket17583: should be clean (prefix fix)
c=tt_bugs_17xxx_ticket17583; d=tools/oracle/cases/$c
for side in off ours; do
  bin=$OFF; [ "$side" = ours ] && bin=$OURS
  timeout 300 "$bin" $(cat "$d/sim.args") -n "$d/net.net.xml" -r "$d/input_routes.rou.xml" --tripinfo-output "$OUT/$c.$side.trip.xml" > /dev/null 2>&1
done
echo "== 17583 tripinfo diff lines: $(diff <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.off.trip.xml") <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.ours.trip.xml") | grep -c '^[<>]')"
# 4) rows flagged as new failures by the last full gate: exit codes + first differing tripinfo/stderr lines
for c in tt_bugs_8xxx_ticket8817 tt_devices_rerouting_stops_stop_same_edge_slack tt_devices_rerouting_weights_stopped_vehicle tt_devices_rerouting_weights_weight_output; do
  d=tools/oracle/cases/$c; net=$d/net.net.xml; [ -f "$net" ] || net=$d/ref_simple.net.xml
  rou=$d/routes.rou; for f in input_routes2.rou.xml input_routes.rou.xml; do [ -f "$d/$f" ] && rou=$d/$f && break; done
  for side in off ours; do
    bin=$OFF; [ "$side" = ours ] && bin=$OURS
    timeout 300 "$bin" $(cat "$d/sim.args" 2>/dev/null) -n "$net" -r "$rou" --tripinfo-output "$OUT/$c.$side.trip.xml" > /dev/null 2> "$OUT/$c.$side.err"; echo $? > "$OUT/$c.$side.rc"
  done
  echo "== FLAG $c rc official=$(cat $OUT/$c.off.rc) ours=$(cat $OUT/$c.ours.rc)"
  diff <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.off.trip.xml" 2>/dev/null) <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.ours.trip.xml" 2>/dev/null) | head -n 4 | cut -c1-250 | sed 's/^/   trip /'
  diff "$OUT/$c.off.err" "$OUT/$c.ours.err" | head -n 4 | cut -c1-250 | sed 's/^/   err  /'
done
# 5) complex rows touched by my stop/estimate fixes (compact: result line + first differing console lines)
for t in vehicle/stopDelay_waypoint vehicle/setStop/newStop_fast_forced vehicle/setStop_default_state; do
  echo "== COMPLEX $t: $(COMPLEX_OUT="$OUT/cx" COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -a "^PASS\|^FAIL" | head -n 2 | cut -c1-260 | tr '
' ' ')"
done
# 6) verbatim OFFICIAL stderr/stdout of the three stale-reference rows (harness invocation), to regenerate ref.errors/ref.output
for c in tt_spec_arrival_errors_lane_given10 tt_spec_arrival_errors_speed_given100 tt_devices_ssm_bugs_ticket5231; do
  echo "== OFFICIAL $c rc=$(cat $OUT/$c.e.off.rc) stderr_lines=$(wc -l < $OUT/$c.e.off.err) stdout_bytes=$(wc -c < $OUT/$c.e.off.out)"
  echo "-- BEGIN stderr"; sed 's/^/|/' "$OUT/$c.e.off.err"; echo "-- END stderr"
  echo "-- BEGIN stdout"; sed 's/^/|/' "$OUT/$c.e.off.out"; echo "-- END stdout"
done
echo "AGENT RUN DONE"
