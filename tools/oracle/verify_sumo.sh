#!/bin/bash
# End-to-end SUMO verification loop for the junction case:
#   build both binaries, run the C++ reference through WSL and the Rust port locally,
#   then diff tripinfo / fcd / statistics.
set -u
cd /c/Users/karti/sumo-rs || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-main
export CARGO_BUILD_JOBS=8

CASE=${1:-junction}
OUT=/c/Users/karti/Temp/verify_$CASE
mkdir -p "$OUT"
CASE_WSL="/mnt/c/Users/karti/sumo-rs/tools/oracle/cases/$CASE"

echo "== build"
cargo build -q -p sumo -p netconvert 2>&1 | grep -v warning | tail -3

echo "== reference (C++)"
wsl -e bash -lc "SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/sumo \
  -n $CASE_WSL/ref_simple.net.xml -r $CASE_WSL/routes.rou \
  --tripinfo-output $CASE_WSL/../${CASE}_ref_ti.xml --fcd-output $CASE_WSL/../${CASE}_ref_fcd.xml \
  --statistic-output $CASE_WSL/../${CASE}_ref_stat.xml --no-step-log true" >"$OUT/ref.log" 2>&1
echo "   exit $?"

echo "== candidate (Rust)"
"$CARGO_TARGET_DIR/debug/sumo.exe" -n "tools/oracle/cases/$CASE/ref_simple.net.xml" \
  -r "tools/oracle/cases/$CASE/routes.rou" \
  --tripinfo-output "$OUT/cand_ti.xml" --fcd-output "$OUT/cand_fcd.xml" \
  --statistic-output "$OUT/cand_stat.xml" --no-step-log true >"$OUT/cand.log" 2>&1
echo "   exit $?  last: $(tail -1 "$OUT/cand.log")"

REF_TI="tools/oracle/cases/${CASE}_ref_ti.xml"
REF_FCD="tools/oracle/cases/${CASE}_ref_fcd.xml"
REF_STAT="tools/oracle/cases/${CASE}_ref_stat.xml"

echo "== tripinfo"
[ -f "$REF_TI" ] && python tools/oracle/compare_sim.py --tripinfo "$REF_TI" "$OUT/cand_ti.xml" | tail -4 || echo "   no reference tripinfo"
echo "== fcd"
[ -f "$REF_FCD" ] && python tools/oracle/compare_sim.py --fcd "$REF_FCD" "$OUT/cand_fcd.xml" | tail -4 || echo "   no reference fcd"
echo "== statistics"
[ -f "$REF_STAT" ] && python tools/oracle/compare_stat.py "$REF_STAT" "$OUT/cand_stat.xml" | tail -4 || echo "   no reference statistics"
