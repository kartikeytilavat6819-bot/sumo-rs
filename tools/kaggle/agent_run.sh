#!/usr/bin/env bash
# oc98 probe (temporary): taxi rows official vs ours consoles
OUT=/tmp/oc98; mkdir -p "$OUT"
for t in vehicle/taxi/basic vehicle/taxi/case1_redispatch_after_pickup; do
  COMPLEX_OUT="$OUT/cx" COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -a "^PASS\|^FAIL" | head -n 2 | cut -c1-300
  for side in official ours; do
    echo "-- $t $side console (first 26 lines, no retry noise):"
    grep -a -v "^ *Retrying\|UserWarning" "$OUT/cx/$t/$side/console.txt" 2>/dev/null | head -n 26 | cut -c1-230 | sed 's/^/     /'
  done
done
# vehroutes_transportable_write_unfinished_person: official vs ours, run like run_all.sh (-n ref_simple.net.xml -r routes.rou + sim.args)
c=tt_output_vehroutes_transportable_write_unfinished_person; d=tools/oracle/cases/$c
for side in off ours; do
  bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
  timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.$side.out" 2> "$OUT/$c.$side.err"; echo $? > "$OUT/$c.$side.rc"
done
echo "== WU $c rc official=$(cat $OUT/$c.off.rc) ours=$(cat $OUT/$c.ours.rc)"
for k in err out; do
  echo "-- std$k official:"; sed -E 's/[(][0-9]+ms[)]/(Nms)/' "$OUT/$c.off.$k" | tail -n 6 | cut -c1-200 | sed 's/^/     /'
  echo "-- std$k ours:";     sed -E 's/[(][0-9]+ms[)]/(Nms)/' "$OUT/$c.ours.$k" | tail -n 6 | cut -c1-200 | sed 's/^/     /'
done
echo "AGENT RUN DONE"
