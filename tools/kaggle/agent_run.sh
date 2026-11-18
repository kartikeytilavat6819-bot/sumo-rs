#!/bin/bash
# Gate diagnostic for oc54: first differing field for each imported devices row.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc54-spot
rm -rf "$OUT"; mkdir -p "$OUT"

for d in tools/oracle/cases/tt_devices_*/; do
  c=$(basename "$d")
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout 25 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true $flags \
      >"$OUT/$c.log" 2>&1
  rc=$?
  if [ "$rc" -ne 0 ]; then
    printf '%-62s rc=%s %s\n' "$c" "$rc" "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-80)"
    continue
  fi
  d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
  d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" 2>&1 | head -1)
  d3=$(python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" 2>&1 | head -1)
  printf '%-62s ti=%s fcd=%s stat=%s\n' "$c" "${d1:-OK}" "${d2:-OK}" "${d3:-OK}"
done
