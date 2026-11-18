#!/bin/bash
# oc37 shortcut audit measurement
set -u
echo "TZ=${TZ:-<unset>} local=$(date '+%Y-%m-%d %H:%M:%S %Z') utc=$(date -u '+%Y-%m-%d %H:%M:%S')"
OUT=${NETOPTS2_OUT:-/tmp/netopts2_oc37}
rm -rf "$OUT"
NETOPTS2_OUT="$OUT" timeout 900 bash tools/oracle/run_netopts_new.sh >/dev/null 2>&1
echo "=== netopts_new summary ==="
cat "$OUT/summary.tsv" 2>/dev/null
echo "=== netopts_new non-PASS ==="
grep -v $'\tPASS\t' "$OUT/summary.tsv" 2>/dev/null | head -20
echo "=== opendrive dates ==="
grep -h '<header' "$OUT/opendriveOutput/cpp_out/net.xodr" "$OUT/opendriveOutput/rust_out/net.xodr" 2>/dev/null | sed 's/.*date="\([^"]*\)".*/\1/'
echo "=== opendrive first diff ==="
diff <(sed '/generated on/d' "$OUT/opendriveOutput/cpp_out/net.xodr" 2>/dev/null) <(sed '/generated on/d' "$OUT/opendriveOutput/rust_out/net.xodr" 2>/dev/null) | head -20
