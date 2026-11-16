#!/bin/bash
# UTM net stage: netconvert WITHOUT --simple-projection on OSM cases, compared with the untouched C++ SUMO.
# run_all.sh always passes --simple-projection and is not changed; this is the default-user path.
# Usage: bash tools/oracle/run_utm.sh [--refs]   (--refs regenerates ref_utm.net.xml via WSL netconvert)
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target-x07}
BIN="$CARGO_TARGET_DIR/debug"
CASES="real signal city_plano city_munich"
OUT=${SCORE_OUT:-/c/Users/karti/Temp/score}/utm-$$
mkdir -p "$OUT"
PASS=0; FAIL=0

if [ "${1:-}" = "--refs" ]; then
  # C++ reference from the untouched SUMO install in WSL (never ~/sumo-src)
  wroot=$(echo "$PWD" | sed 's#^/c/#/mnt/c/#; s#^C:/#/mnt/c/#')
  for c in $CASES; do
    wsl -e bash -c "export SUMO_HOME=\$(~/sumo-pristine/bin/python -c 'import sumo; print(sumo.SUMO_HOME)'); cd '$wroot/tools/oracle/cases/$c' && ~/sumo-pristine/bin/netconvert --osm-files input.osm -o ref_utm.net.xml" >/dev/null 2>&1
    echo "ref $c: $(ls "tools/oracle/cases/$c/ref_utm.net.xml" 2>/dev/null || echo MISSING)"
  done
fi

for c in $CASES; do
  dir="tools/oracle/cases/$c"
  if [ ! -f "$dir/input.osm" ] || [ ! -f "$dir/ref_utm.net.xml" ]; then
    echo "$c net SKIP no input.osm or ref_utm.net.xml"; continue
  fi
  "$BIN/netconvert.exe" --osm-files "$dir/input.osm" -o "$OUT/$c.net.xml" >/dev/null 2>&1
  ref_loc=$(grep -o '<location[^>]*>' "$dir/ref_utm.net.xml" | grep -o 'projParameter="[^"]*"')
  our_loc=$(grep -o '<location[^>]*>' "$OUT/$c.net.xml" 2>/dev/null | grep -o 'projParameter="[^"]*"')
  if python tools/oracle/compare_net.py "$dir/ref_utm.net.xml" "$OUT/$c.net.xml" >"$OUT/$c.net.log" 2>&1 && [ "$ref_loc" = "$our_loc" ]; then
    echo "$c net PASS $our_loc"; PASS=$((PASS+1))
  else
    echo "$c net FAIL proj ref=[$ref_loc] ours=[$our_loc] $(grep SUMMARY "$OUT/$c.net.log" | head -1)"; FAIL=$((FAIL+1))
  fi
done
echo "utm net PASS=$PASS FAIL=$FAIL (logs in $OUT)"
