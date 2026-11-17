#!/bin/bash
# oc05 diagnostics for failing netopts_new rows.
B=$(cd "$(dirname "$0")/../oracle" && pwd)
ROOT=$(cd "$B/../.." && pwd)
cd "$ROOT" || exit 1
OUT=${NETOPTS2_OUT:-/tmp/netopts2oc05}
export NETOPTS2_OUT=$OUT
export RUST_BACKTRACE=1
export NETOPTS2_JOBS=1
bash "$B/run_netopts_new.sh" geomSplit tlsGuessJoining plainOutputLanes ptstopNoBidi railwayTopologyOutput
echo "======== geomSplit rust.log (panic/backtrace) ========"
cat "$OUT/geomSplit/rust.log" | head -80
echo "======== tlsGuessJoining joinedG diff ========"
diff <(grep -n 'joinedG_150093074_6734581854' "$OUT/tlsGuessJoining/cpp.net.xml") <(grep -n 'joinedG_150093074_6734581854' "$OUT/tlsGuessJoining/rust.net.xml") | head -40
echo "======== tlsGuessJoining tlLogic count ========"
echo "cpp=$(grep -c '<tlLogic' "$OUT/tlsGuessJoining/cpp.net.xml") rust=$(grep -c '<tlLogic' "$OUT/tlsGuessJoining/rust.net.xml")"
echo "======== plainOutputLanes net.con.xml diff ========"
diff "$OUT/plainOutputLanes/cpp_out/plain/net.con.xml" "$OUT/plainOutputLanes/rust_out/plain/net.con.xml" | head -40
echo "======== ptstopNoBidi ptstop.xml diff ========"
diff "$OUT/ptstopNoBidi/cpp_out/ptstop.xml" "$OUT/ptstopNoBidi/rust_out/ptstop.xml" | head -40
echo "======== railwayTopologyOutput topo.xml diff ========"
diff "$OUT/railwayTopologyOutput/cpp_out/topo.xml" "$OUT/railwayTopologyOutput/rust_out/topo.xml" | head -40
