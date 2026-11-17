#!/bin/bash
# rb02 diagnostic (removed before merge): veh 21 best lanes at steps 40..44, Rust vs instrumented C++
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
BIN="$CARGO_TARGET_DIR/debug"
D=tools/oracle/cases/city_paris_etoile
O=/tmp/rb02; rm -rf $O; mkdir -p $O
EXTRA=$(cat "$D/sim.args")
"$BIN/sumo.exe" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --fcd-output "$O/fcd.xml" \
    --tripinfo-output "$O/ti.xml" --no-step-log true $EXTRA >"$O/sumo.log" 2>&1
echo "rust exit=$?"
S=/home/gate/sumo-cpp/src/microsim/MSVehicle.cpp
echo "== VM src: $(md5sum $S 2>&1) $(file -b $S 2>&1)"
sed -n '6023,6028p' $S | cat -A | cut -c1-120
git -C /home/gate/sumo-cpp status --short 2>&1 | head -5
echo "== repo patch head"; head -8 tools/kaggle/rb02_ubl.patch | cat -A | cut -c1-100
sed 's/\r$//' tools/kaggle/rb02_ubl.patch > $O/rb02_ubl.lf.patch
P=$(bash tools/gcloud/vm_cpp_trace.sh rb02 $O/rb02_ubl.lf.patch) || { echo "cpp trace build failed"; exit 1; }
"$P" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --fcd-output "$O/cfcd.xml" \
    --tripinfo-output "$O/cti.xml" --no-step-log true $EXTRA >"$O/cpp.log" 2>&1
echo "cpp exit=$?"
echo "== RUST RB02"; grep RB02 "$O/sumo.log" | head -60
echo "== CPP RB02"; grep RB02 "$O/cpp.log" | head -60
python tools/oracle/compare_sim.py --fcd "$D/ref.fcd.xml" "$O/fcd.xml" 2>&1 | head -3
