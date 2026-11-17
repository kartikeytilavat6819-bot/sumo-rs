#!/bin/bash
# lo01 measurement (remove before merge): lc_opposite panic backtrace + sublane_bikes FCD diff vs C++ reference
cd "$(dirname "$0")/../.." || exit 1
B=/tmp/target/debug
O=/tmp/lo01_out
mkdir -p "$O"
for c in lc_opposite sublane_bikes; do
  d=tools/oracle/cases/$c
  RUST_BACKTRACE=1 "$B/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --tripinfo-output "$O/$c.trip.xml" --fcd-output "$O/$c.fcd.xml" --statistic-output "$O/$c.stat.xml" --no-step-log true >"$O/$c.out" 2>"$O/$c.err"
  echo "$c ours exit $?"
  grep -m1 -A2 'panicked' "$O/$c.err" | cut -c1-240
  grep -E 'sumo_core|at \./|ms_lane' "$O/$c.err" | head -16 | cut -c1-200
  echo "$c fcd diff lines=$(diff "$d/ref.fcd.xml" "$O/$c.fcd.xml" | wc -l) trip diff lines=$(diff "$d/ref.tripinfo.xml" "$O/$c.trip.xml" | wc -l) stat diff lines=$(diff "$d/ref.stat.xml" "$O/$c.stat.xml" | wc -l)"
  diff "$d/ref.fcd.xml" "$O/$c.fcd.xml" | head -8 | cut -c1-240
  diff "$d/ref.tripinfo.xml" "$O/$c.trip.xml" | head -6 | cut -c1-240
done
