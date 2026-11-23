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
echo "== 4299 ES1 FCD diff (official '<' vs ours '>'):"
diff -U2 <(grep 'id="ES1"' "$OUT/$c.off.fcd.xml") <(grep 'id="ES1"' "$OUT/$c.ours.fcd.xml") | head -n 14 | cut200
echo "== 4299 vehicles with a different FCD line count:"
diff <(grep -o 'id="[^"]*"' "$OUT/$c.off.fcd.xml" | sort | uniq -c) <(grep -o 'id="[^"]*"' "$OUT/$c.ours.fcd.xml" | sort | uniq -c) | head -n 6
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
      echo "   std$k DIFF  official:"; grep -a -v "^Loading\|^ *$" "$OUT/$c.e.off.$k" | head -n 5 | cut200 | sed 's/^/      /'
      echo "              ours:";     grep -a -v "^Loading\|^ *$" "$OUT/$c.e.ours.$k" | head -n 5 | cut200 | sed 's/^/      /'
    fi
  done
  echo "   ref.errors:"; head -n 3 "$d/ref.errors" 2>/dev/null | cut200 | sed 's/^/      /'
  echo "   ref.output: $(wc -c < "$d/ref.output" 2>/dev/null) bytes: $(head -c 120 "$d/ref.output" 2>/dev/null | tr '\n' '|')"
done

# 3) tt_bugs_17xxx_ticket17583: should be clean (prefix fix)
c=tt_bugs_17xxx_ticket17583; d=tools/oracle/cases/$c
for side in off ours; do
  bin=$OFF; [ "$side" = ours ] && bin=$OURS
  timeout 300 "$bin" $(cat "$d/sim.args") -n "$d/net.net.xml" -r "$d/input_routes.rou.xml" --tripinfo-output "$OUT/$c.$side.trip.xml" > /dev/null 2>&1
done
echo "== 17583 tripinfo diff lines: $(diff <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.off.trip.xml") <(grep -v -e 'generated on' -e 'output value' "$OUT/$c.ours.trip.xml") | grep -c '^[<>]')"
echo "AGENT RUN DONE"
