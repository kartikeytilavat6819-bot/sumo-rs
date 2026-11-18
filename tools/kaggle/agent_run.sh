#!/bin/bash
# Gate diagnostic for oc60: run one or more imported oracle rows with our binary
# and print the first difference against the committed official reference.
# Target cases are listed one-per-line in tools/kaggle/agent_cases.txt.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=tools/oracle/.oc60diag
rm -rf "$OUT"; mkdir -p "$OUT"
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_*' | wc -l)
active=$(find tools/oracle/cases -maxdepth 2 -type f ! -name PENDING -path '*tt_*' -name 'ref.tripinfo.xml' | wc -l)
echo "tt cases=$total active-with-ref=$active"

while read -r c; do
  c=$(echo "$c" | tr -d '\r')
  [ -n "$c" ] || continue
  d=tools/oracle/cases/$c
  [ -f "$d/ref.tripinfo.xml" ] || { echo "$c: NO CASE/REF"; continue; }
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout "${SIM_TIMEOUT:-120}" "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-62s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:|Fatal' "$OUT/$c.log" | cut -c1-140
    continue
  fi
  d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | grep -m1 -E 'DIFF|MISMATCH|SUMMARY|Error' | cut -c1-220)
  echo "tripinfo:${d1:-MATCH}"
  if [ -f "$d/ref.fcd.xml" ]; then
    d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | grep -m1 -E 'DIFF|MISMATCH|SUMMARY|Error' | cut -c1-160)
    echo "  fcd:${d2:-MATCH}"
  fi
done < tools/kaggle/agent_cases.txt
