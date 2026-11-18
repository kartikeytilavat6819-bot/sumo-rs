#!/bin/bash
# Gate diagnostic for oc49: spot-check 4 imported TextTest rows directly.
# Does not re-run the whole scoreboard (the gate already does that).
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/c/Users/karti/AppData/Local/Temp/opencode/oc49-spot
rm -rf "$OUT"; mkdir -p "$OUT"
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_*' | wc -l)
pending=$(find tools/oracle/cases -maxdepth 2 -type f -name PENDING -path '*tt_*' | wc -l)
echo "imported=$total active=$((total - pending))"

for c in tt_basic_routes_simtime_explicite_b0_e400 tt_cf_model_IDM_ticket11498 tt_cf_model_ignoreTypes_idm tt_cf_model_slope_default_model; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  timeout 20 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-58s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-90
  else
    d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
    echo "tripinfo:${d1:-MATCH}"
  fi
done
