#!/bin/bash
# netgenerate and polyconvert oracles: C++ 1.28.0 vs ours, rows "<name> <options>" (@D = per-side output dir).
# netgenerate: net compared with compare_net.py; polyconvert: output compared after dropping the 'generated on' line.
# Prints "netgenerate oracle: PASS=x of y" / "polyconvert oracle: PASS=x of y" and the failing rows.
# Run from the repo root on the gate VM (SUMO_PRISTINE_BIN, CARGO_TARGET_DIR set). Usage: run_gen_poly.sh [netgenerate|polyconvert]
B=$(cd "$(dirname "$0")" && pwd)
cd "$B/../.." || exit 1
C=$B/cases
OUT=${GENPOLY_OUT:-/tmp/genpoly}
rm -rf "$OUT"; mkdir -p "$OUT"
R=$C/real/input.osm; RN=$C/real/ref_simple.net.xml; SH=$C/polyconvert/shapes.xml
NG=(); PC=()
ng() { NG+=("$1 $2"); }
pc() { PC+=("$1 $2"); }
ng gridBasic "--grid --grid.number 3 --grid.length 100"
ng gridAttach "--grid --grid.x-number 4 --grid.y-number 2 --grid.length 80 --grid.attach-length 50"
ng gridDefaultJunctionType "--grid --grid.number 3 --default-junction-type traffic_light"
ng gridTlsGuess "--grid --grid.number 3 --grid.length 120 --tls.guess true --default-junction-type priority"
ng gridTurnLanes "--grid --grid.number 3 --grid.length 150 --turn-lanes 1 --turn-lanes.length 30 --default.lanenumber 2"
ng gridNoInternal "--grid --grid.number 3 --no-internal-links true"
ng spiderBasic "--spider --spider.arm-number 6 --spider.circle-number 3 --spider.space-radius 100"
ng spiderOmitCenter "--spider --spider.arm-number 5 --spider.circle-number 2 --spider.omit-center true --spider.attach-length 40"
ng randBasic "--rand --rand.iterations 20 --seed 42"
ng randGrid "--rand --rand.iterations 30 --rand.grid true --seed 7"
ng gridTlsLayout "--grid --grid.number 3 --default-junction-type traffic_light --tls.layout incoming"
pc osmDefault "--osm-files $R -o @D/out.xml"
pc osmNet "--osm-files $R --net-file $RN -o @D/out.xml"
pc osmPruneInNet "--osm-files $R --net-file $RN --prune.in-net true --prune.in-net.offsets 10,10,-10,-10 -o @D/out.xml"
pc osmOffset "--osm-files $R --net-file $RN --offset.x 100 --offset.y -50 -o @D/out.xml"
pc osmFullTypeName "--osm-files $R --net-file $RN --osm.keep-full-type true --osm.use-name true -o @D/out.xml"
pc osmTypemapProj "--osm-files $R --type-file \$SUMO_HOME/data/typemap/osmPolyconvertRail.typ.xml --simple-projection true -o @D/out.xml"
pc xmlShapes "--xml-files $SH -o @D/out.xml"
pc xmlPruneBoundary "--xml-files $SH --prune.boundary 0,0,500,500 -o @D/out.xml"
pc xmlFlatten "--xml-files $SH --flatten true --poi-layer-offset 2 --prefix x_ -o @D/out.xml"

run_ng() { # idx
  local n opts; read -r n opts <<< "${NG[$1]}"; local d=$OUT/ng/$n; mkdir -p "$d"
  local -a a; read -ra a <<< "$opts"
  timeout 300 "$CARGO_TARGET_DIR/debug/netgenerate" "${a[@]}" -o "$d/rust.net.xml" > "$d/rust.log" 2>&1; local rr=$?
  timeout 300 "$SUMO_PRISTINE_BIN/netgenerate" "${a[@]}" -o "$d/cpp.net.xml" > "$d/cpp.log" 2>&1; local cr=$?
  local V=PASS D=ok LN="s#$d/cpp#OUT#g; s#$d/rust#OUT#g; s/\r\$//"
  if grep -q panicked "$d/rust.log"; then V=FAIL; D="PANIC $(grep -m1 -A1 panicked "$d/rust.log" | tr '\n' ' ')"
  elif [ $rr -ne $cr ]; then V=FAIL; D="exit rust=$rr cpp=$cr: $(grep -m1 -i error "$d/rust.log")"
  elif [ $cr -eq 0 ]; then
    python3 "$B/compare_net.py" "$d/cpp.net.xml" "$d/rust.net.xml" > "$d/net.log" 2>&1 || { V=FAIL; D="$(head -1 "$d/net.log") | $(grep SUMMARY "$d/net.log")"; }
    if [ $V = PASS ] && ! diff -q <(sed "$LN" "$d/cpp.log") <(sed "$LN" "$d/rust.log") >/dev/null; then V=FAIL; D="log differs: $(diff <(sed "$LN" "$d/cpp.log") <(sed "$LN" "$d/rust.log") | head -2 | tr '\n' ' ')"; fi
  fi
  printf '%s\t%s\t%s\n' "$n" "$V" "$D" >> "$OUT/ng.tsv"
}
run_pc() {
  local n opts; read -r n opts <<< "${PC[$1]}"; local d=$OUT/pc/$n; mkdir -p "$d"
  local -a ca ra
  eval "ca=(${opts//@D/$d/cpp})"; eval "ra=(${opts//@D/$d/rust})"
  mkdir -p "$d/cpp" "$d/rust"
  timeout 300 "$CARGO_TARGET_DIR/debug/polyconvert" "${ra[@]}" > "$d/rust.log" 2>&1; local rr=$?
  timeout 300 "$SUMO_PRISTINE_BIN/polyconvert" "${ca[@]}" > "$d/cpp.log" 2>&1; local cr=$?
  local V=PASS D=ok norm="s#$d/cpp#OUT#g; s#$d/rust#OUT#g; /generated on/d; s/\r\$//"
  if grep -q panicked "$d/rust.log"; then V=FAIL; D="PANIC $(grep -m1 -A1 panicked "$d/rust.log" | tr '\n' ' ')"
  elif [ $rr -ne $cr ]; then V=FAIL; D="exit rust=$rr cpp=$cr: $(grep -m1 -i error "$d/rust.log")"
  elif [ $cr -eq 0 ] && ! diff -q <(sed -e "$norm" "$d/cpp/out.xml") <(sed -e "$norm" "$d/rust/out.xml") >/dev/null 2>&1; then
    V=FAIL; D="out.xml differs: $(diff <(sed -e "$norm" "$d/cpp/out.xml") <(sed -e "$norm" "$d/rust/out.xml") 2>&1 | head -3 | tr '\n' ' ')"
  fi
  printf '%s\t%s\t%s\n' "$n" "$V" "$D" >> "$OUT/pc.tsv"
}
report() { # tool tsv
  if [ ! -s "$2" ]; then echo "$1 oracle: PASS=0 of 0"; return; fi
  echo "$1 oracle: PASS=$(grep -c $'\tPASS\t' "$2") of $(wc -l < "$2")"; grep -v $'\tPASS\t' "$2" | cut -c1-300
}
: > "$OUT/ng.tsv"; : > "$OUT/pc.tsv"
WANT=${1:-all}
for t in ng pc; do
  [ "$WANT" != all ] && [ "$WANT" != "$( [ $t = ng ] && echo netgenerate || echo polyconvert)" ] && continue
  if [ $t = ng ]; then tool=netgenerate; cnt=${#NG[@]}; else tool=polyconvert; cnt=${#PC[@]}; fi
  if [ ! -x "$SUMO_PRISTINE_BIN/$tool" ]; then echo "$tool oracle: pristine $SUMO_PRISTINE_BIN/$tool MISSING"; continue; fi
  if [ ! -x "$CARGO_TARGET_DIR/debug/$tool" ]; then echo "$tool oracle: our binary MISSING"; continue; fi
  for ((i=0;i<cnt;i++)); do run_$t $i; done
  report $tool "$OUT/$t.tsv"
done
