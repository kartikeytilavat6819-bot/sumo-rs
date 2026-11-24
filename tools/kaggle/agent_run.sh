#!/usr/bin/env bash
# oc121 probe (temporary)
OUT=/tmp/oc121; mkdir -p "$OUT"
for c in tt_output_route_probe_two_same tt_output_route_probe_vTypes; do
  d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    timeout 120 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"; echo "== $c $side rc=$?"
    head -n 4 "$OUT/$c.$side.err" | cut -c1-250; head -n 3 "$OUT/$c.$side.out" | cut -c1-200
  done
done
d=tools/oracle/cases/duarouter_itt_duarouter_meta_write_config
for side in off ours; do
  bin=${SUMO_PRISTINE_BIN}/duarouter; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/duarouter
  a=$(sed "s#@DIR@#$OUT/$side#g" $d/args); mkdir -p $OUT/$side
  $bin $a >/dev/null 2>&1; echo "== cfg $side"; sed -n 1,14p $OUT/$side/config.cfg
done
echo "AGENT RUN DONE"
