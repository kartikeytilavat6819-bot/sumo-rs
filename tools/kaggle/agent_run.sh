#!/usr/bin/env bash
# oc98 probe (temporary): official 1.28.0 output of the five rail overtaking_follower cases, with the exact
# run_all.sh expected-error command (-n ref_simple.net.xml -r routes.rou --no-step-log true + sim.args)
OUT=/tmp/oc98; mkdir -p "$OUT"
for s in arrival_delay_low delay_high delay_high_maxDelay_default delay_high_maxDelay_high delay_low; do
  c=tt_rail_rerouter_overtaking_follower_$s; d=tools/oracle/cases/$c
  timeout 300 "${SUMO_PRISTINE_BIN}/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" --no-step-log true $(cat "$d/sim.args") > "$OUT/$c.out" 2> "$OUT/$c.err"
  rc=$?
  echo "== $c rc=$rc stdout=$(wc -c < "$OUT/$c.out")B/$(wc -l < "$OUT/$c.out")L stderr=$(wc -c < "$OUT/$c.err")B/$(wc -l < "$OUT/$c.err")L"
  echo "-- stderr (exact, cat -A):"; head -n 12 "$OUT/$c.err" | cut -c1-300 | cat -A | sed 's/^/     /'
  echo "-- stdout (first 6, cat -A):"; head -n 6 "$OUT/$c.out" | cut -c1-300 | cat -A | sed 's/^/     /'
done
echo "AGENT RUN DONE"
