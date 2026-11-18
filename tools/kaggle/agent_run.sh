#!/bin/bash
# Diagnose the depart regression: run 2 variants with our sumo, print rc + stderr + compare diffs.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
OUT=${OUT:-/tmp/agent_run_depart}
mkdir -p "$OUT"
for n in pos_base lane_1 speed_0; do
  r="tools/oracle/cases/depart/$n.rou"
  o="$OUT/$n"; mkdir -p "$o"; rm -f "$o"/*
  "$BIN" -n tools/oracle/cases/junction/ref_simple.net.xml -r "$r" \
    --tripinfo-output "$o/tripinfo.xml" --statistic-output "$o/statistic.xml" \
    --seed 42 --end 120 --no-step-log >"$o/stdout.txt" 2>"$o/stderr.txt"
  echo "== $n rc=$?"
  echo "-- stderr head:"; head -5 "$o/stderr.txt"
  echo "-- stdout head:"; head -5 "$o/stdout.txt"
  ref="tools/oracle/cases/depart/ref/$n"
  if [ -f "$ref/stderr.txt" ]; then
    echo "-- ref stderr head:"; head -3 "$ref/stderr.txt"
  fi
  if [ -f "$ref/tripinfo.xml" ] && [ -f "$o/tripinfo.xml" ]; then
    echo "-- tripinfo diff head:"
    diff <(head -20 "$ref/tripinfo.xml") <(head -20 "$o/tripinfo.xml") | head -20
  fi
done
