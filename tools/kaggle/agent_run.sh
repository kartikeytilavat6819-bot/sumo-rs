#!/usr/bin/env bash
# oc98 probe (temporary): taxi rows official vs ours consoles
OUT=/tmp/oc98; mkdir -p "$OUT"
for t in vehicle/taxi/basic vehicle/taxi/case1_redispatch_after_pickup; do
  COMPLEX_OUT="$OUT/cx" COMPLEX_FILTER="$t" bash tools/oracle/run_complex_suite.sh 2>&1 | grep -a "^PASS\|^FAIL" | head -n 2 | cut -c1-300
  for side in official ours; do
    echo "-- $t $side console (first 26 lines, no retry noise):"
    grep -a -v "^ *Retrying\|UserWarning" "$OUT/cx/$t/$side/console.txt" 2>/dev/null | head -n 26 | cut -c1-230 | sed 's/^/     /'
  done
done
echo "AGENT RUN DONE"
