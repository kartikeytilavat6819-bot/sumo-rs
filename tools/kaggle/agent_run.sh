#!/bin/bash
# oc05 diagnostics for failing netopts_new rows.
B=$(cd "$(dirname "$0")/../oracle" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
OUT=${NETOPTS2_OUT:-/tmp/netopts2oc05b}
export NETOPTS2_OUT=$OUT
export RUST_BACKTRACE=1
export NETOPTS2_JOBS=1
bash "$B/run_netopts_new.sh" geomSplit tlsGuessJoining plainOutputLanes ptstopNoBidi railwayTopologyOutput
echo "======== geomSplit status ========"
grep -m1 -E 'panicked|Success' "$OUT/geomSplit/rust.log"
echo "======== tlsGuessJoining determinism (8 runs, TL counts) ========"
RUST=${RUST_NETCONVERT:-$CARGO_TARGET_DIR/debug/netconvert}
for k in $(seq 1 8); do
  $RUST --simple-projection true --osm-files tools/oracle/cases/city_plano/input.osm --tls.guess true --tls.guess.joining true -o "$OUT/tlsr$k.xml" >/dev/null 2>&1
  echo "run$k tl150=$(grep -c 'tlLogic id=\"joinedG_150093074' "$OUT/tlsr$k.xml") tl540=$(grep -c 'tlLogic id=\"joinedG_5407273356' "$OUT/tlsr$k.xml") total=$(grep -c '<tlLogic' "$OUT/tlsr$k.xml")"
done
echo "======== tlsGuessJoining diff ========"
diff <(grep -n 'joinedG_150093074\|joinedG_5407273356' "$OUT/tlsGuessJoining/cpp.net.xml") <(grep -n 'joinedG_150093074\|joinedG_5407273356' "$OUT/tlsGuessJoining/rust.net.xml") | head -30
echo "======== normalized extra-file diffs ========"
for row in plainOutputLanes:plain/net.con.xml ptstopNoBidi:ptstop.xml railwayTopologyOutput:topo.xml; do
  n=${row%%:*}; f=${row#*:}
  d=$OUT/$n
  norm='s#'"$d"'/cpp_out#OUT#g; s#'"$d"'/rust_out#OUT#g; /generated on/d; s/\r$//; s#'"$n"'/cpp\.net\.xml#OUT.net.xml#g; s#'"$n"'/rust\.net\.xml#OUT.net.xml#g'
  echo "--- $n $f ---"
  diff <(sed -e "$norm" "$d/cpp_out/$f") <(sed -e "$norm" "$d/rust_out/$f") | head -20
done
