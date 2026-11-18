#!/bin/bash
# Gate diagnostic for oc49: spot-check a few imported TextTest rows directly.
# Deliberately does NOT re-run the whole scoreboard (the gate already does that).
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

for d in tools/oracle/cases/tt_*/; do
  c=$(basename "$d")
  [ -f "$d/PENDING" ] && continue
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args")
  timeout 30 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-60s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-90
  else
    diff=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -1)
    echo "ti:${diff:-MATCH}"
  fi
done
