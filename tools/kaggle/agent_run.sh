#!/bin/bash
# rp01 measurement only: first diffs for routeprobe and bluelight against committed C++ refs. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
B="$CARGO_TARGET_DIR/debug"
for c in routeprobe bluelight; do
  dir=tools/oracle/cases/$c
  OUT=/tmp/rp01/$c
  rm -rf "$OUT"; mkdir -p "$OUT"
  EXTRA=""; [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
  "$B/sumo" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true $EXTRA >"$OUT/sumo.log" 2>&1
  echo "== $c sumo exit=$?"
  grep -m2 -E 'panicked at|Error:' -A1 "$OUT/sumo.log" | cut -c1-200
  python3 tools/oracle/compare_sim.py --tripinfo "$dir/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -12
  [ -f "$dir/ref.fcd.xml" ] && python3 tools/oracle/compare_sim.py --fcd "$dir/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -6
  [ -f "$dir/ref.stat.xml" ] && python3 tools/oracle/compare_stat.py "$dir/ref.stat.xml" "$OUT/stat.xml" 2>&1 | tail -4
done
