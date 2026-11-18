#!/bin/bash
# oc28 CCH diagnostics: duarouter CCH oracle rows + sim CCH tripinfo diff.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug
C=tools/oracle/cases/cch_reroute
OUT=/tmp/oc28_cch
mkdir -p $OUT

echo "=== duarouter CCH oracle ==="
OUT=/tmp/oc28_dua bash tools/oracle/run_duarouter.sh duarouter_cch duarouter_cch_alt duarouter_cch_weights 2>&1 | tail -8
echo "--- results ---"
cat /tmp/oc28_dua/results.tsv 2>/dev/null

echo "=== sim CCH (cch_reroute) ==="
EXTRA=$(cat "$C/sim.args")
timeout 300 "$BIN/sumo" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
  --tripinfo-output $OUT/ti.xml --fcd-output $OUT/fcd.xml --statistic-output $OUT/stat.xml \
  --no-step-log true $EXTRA > $OUT/sumo.log 2>&1
echo "ours exit=$?"
grep -m1 -A2 -E 'panicked at|Error:' $OUT/sumo.log | head -3
python3 tools/oracle/compare_sim.py --tripinfo "$C/ref.tripinfo.xml" $OUT/ti.xml > $OUT/ti.diff 2>&1
echo "ti compare exit=$?"
head -12 $OUT/ti.diff
