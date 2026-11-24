#!/usr/bin/env bash
# oc98 probe (temporary): the five rail overtaking_follower cases through the run_all.sh *simulation* stage
# (-n ref_simple.net.xml -r routes.rou --tripinfo/--statistic-output --no-step-log true + sim.args), official and ours,
# each compared with the stored ref.tripinfo.xml / ref.stat.xml by compare_sim.py / compare_stat.py
OUT=/tmp/oc98; mkdir -p "$OUT"
for s in arrival_delay_low delay_high delay_high_maxDelay_default delay_high_maxDelay_high delay_low; do
  c=tt_rail_rerouter_overtaking_follower_$s; d=tools/oracle/cases/$c
  for side in off ours; do
    bin=${SUMO_PRISTINE_BIN}/sumo; [ "$side" = ours ] && bin=${CARGO_TARGET_DIR}/debug/sumo
    timeout 300 "$bin" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --tripinfo-output "$OUT/$c.$side.ti.xml" \
        --statistic-output "$OUT/$c.$side.stat.xml" --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.$side.log" 2>&1
    echo "$?" > "$OUT/$c.$side.rc"
  done
  echo "== $c rc official=$(cat $OUT/$c.off.rc) ours=$(cat $OUT/$c.ours.rc)"
  for side in off ours; do
    echo "-- $side tripinfo vs stored ref:"; python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.$side.ti.xml" 2>&1 | head -n 4 | cut -c1-260 | sed 's/^/     /'
    echo "-- $side stat vs stored ref:";     python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/$c.$side.stat.xml" 2>&1 | head -n 4 | cut -c1-260 | sed 's/^/     /'
  done
  echo "-- ours log (first 3):"; head -n 3 "$OUT/$c.ours.log" | cut -c1-260 | sed 's/^/     /'
done
echo "AGENT RUN DONE"
