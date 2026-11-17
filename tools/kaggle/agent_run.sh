#!/bin/bash
# rb02 diagnostic (removed before merge): city_paris_etoile FCD first diff + veh 21 around t=40..43
cd "$(dirname "$0")/../.." || exit 1
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/target}
BIN="$CARGO_TARGET_DIR/debug"
D=tools/oracle/cases/city_paris_etoile
O=/tmp/rb02; rm -rf $O; mkdir -p $O
EXTRA=$(cat "$D/sim.args")
"$BIN/sumo.exe" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --fcd-output "$O/fcd.xml" \
    --tripinfo-output "$O/ti.xml" --no-step-log true $EXTRA >"$O/sumo.log" 2>&1
echo "exit=$? lines=$(wc -l < $O/fcd.xml)"
python tools/oracle/compare_sim.py --fcd "$D/ref.fcd.xml" "$O/fcd.xml" 2>&1 | head -8
for f in "$D/ref.fcd.xml" "$O/fcd.xml"; do
  echo "== $f"
  awk 'match($0,/time="[^"]*"/){t=substr($0,RSTART+6,RLENGTH-7)} /id="21"/ && t>=40 && t<=43 {print t": "$0}' "$f" | head -12
done
