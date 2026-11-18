#!/bin/bash
# oc66: measure only the netconvert import TextTest rows I contributed (dlr_navteq, matsim).
cd "$(cd "$(dirname "$0")/../.." && pwd)" || exit 1
R=tools/oracle/import_texttest_net.rows
echo "ROWS FILE: $(grep -c '^row ' "$R") rows; mine: $(grep -cE '^row itt_nc_netconvert_import_(dlr_navteq|matsim)_' "$R")"
NAMES=$(grep -oE 'row itt_nc_netconvert_import_(dlr_navteq|matsim)_[^ ]+' "$R" | awk '{print $2}' | tr -d '\r')
echo "NAMES count: $(echo "$NAMES" | grep -c .)"
O=/tmp/oc66_netopts
rm -rf "$O"
NETOPTS2_OUT="$O" NETOPTS2_JOBS=8 timeout 540 bash tools/oracle/run_netopts_new.sh $NAMES >/dev/null 2>&1
T="$O/summary.tsv"
echo "OC66 NETOPTS: PASS=$(grep -c $'\tPASS\t' "$T" 2>/dev/null) FAIL=$(grep -c $'\tFAIL\t' "$T" 2>/dev/null) PANIC=$(grep -c $'\tPANIC\t' "$T" 2>/dev/null) TOTAL=$(wc -l < "$T" 2>/dev/null)"
echo "=== non-PASS rows (name, verdict, detail) ==="
grep -v $'\tPASS\t' "$T" 2>/dev/null | cut -f1,2,3 | head -60
echo "=== logs for non-PASS rows ==="
for r in $(grep -v $'\tPASS\t' "$T" 2>/dev/null | cut -f1); do
  echo "--- $r cpp.log:"; head -4 "$O/$r/cpp.log" 2>/dev/null | tr '\r' ' '
  echo "--- $r rust.log:"; head -4 "$O/$r/rust.log" 2>/dev/null | tr '\r' ' '
  echo "--- $r net.log:"; head -8 "$O/$r/net.log" 2>/dev/null | tr '\r' ' '
done
