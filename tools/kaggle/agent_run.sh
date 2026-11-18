#!/bin/bash
# Gate helper for oc68: show netconvert/function imported-row results and first diffs.
set -u
B=$(cd "$(dirname "$0")/.." && pwd)/oracle
OUT=${NETOPTS2_OUT:-/tmp/netopts2}
S="$OUT/summary.tsv"
echo "=== FUNCTION ROW VERDICTS ==="
if [ ! -f "$S" ]; then echo "no summary at $S"; exit 0; fi
grep 'itt_nc_netconvert_function_' "$S" | cut -f2 | sort | uniq -c
echo "=== NON-PASS FUNCTION ROWS ==="
grep 'itt_nc_netconvert_function_' "$S" | grep -v $'\tPASS\t' | head -100
echo "=== FIRST FAIL DIFFS ==="
n=0
while IFS=$'\t' read -r name verdict detail; do
  case "$name" in itt_nc_netconvert_function_*) ;; *) continue;; esac
  [ "$verdict" = PASS ] && continue
  echo "### $name ($verdict) $detail"
  d="$OUT/$name"
  [ -f "$d/net.log" ] && head -12 "$d/net.log"
  n=$((n+1)); [ $n -ge 6 ] && break
done < "$S"
echo "=== DONE ==="
