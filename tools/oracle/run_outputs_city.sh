#!/bin/bash
# City-scale output oracle: the six largest city cases by vehicle count (reference tripinfo), every output
# option run live on the official C++ sumo ($REF_BIN) and on ours ($BIN) with the case's sim.args, then
# compared element by element with compare_xml.py (exact). No committed references: run on the VM.
# Usage: bash tools/oracle/run_outputs_city.sh [case ...]
#   ROWS="summary queue" restricts the output rows; OUT (default /tmp/city_outputs) gets $c/$name/{ref,cand}/.
#   BIN default $CARGO_TARGET_DIR/debug/sumo, REF_BIN default $SUMO_PRISTINE_BIN/sumo.
# Sim length: the case's own --end, except netstate and emission, which use a 120 s window (short outputs).
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN=${BIN:-${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo}
REF_BIN=${REF_BIN:-${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}/sumo}
OUT=${OUT:-/tmp/city_outputs}
RESULTS=$OUT/results.tsv
mkdir -p "$OUT"
: > "$RESULTS"
printf 'output\tcase\tverdict\tref_rc\tcand_rc\tdetail\n' >> "$RESULTS"
CASES=${*:-kumbakonam_mixed autobahn_interchange zurich_tram karlsruhe_tram cairo_priority hongkong_lefthand_grid}
CASES=$(for c in $CASES; do printf 'city_%s ' "${c#city_}"; done)

# one sumo run; retries the C++ side because it sometimes dies with "Interrupt signal" and a truncated file
run_side() {  # run_side <bin> <log> <retries> <args...>
  local bin=$1 log=$2 tries=$3; shift 3
  local t rc=0
  for t in $(seq "$tries"); do
    timeout 1500 "$bin" "$@" >"$log" 2>&1; rc=$?
    [ $rc -eq 0 ] && ! grep -q 'Interrupt signal' "$log" && break
  done
  return $rc
}

# row <case> <name> <file> <end> <extra args...>; @DIR@ in the extra args is the per-side output dir
row() {
  local c=$1 name=$2 file=$3 end=$4; shift 4
  local cdir=tools/oracle/cases/$c sub dr dc rrc crc verdict detail="" sim base=() a rx=()
  [ -z "${ROWS:-}" ] || [[ " $ROWS " == *" $name "* ]] || return 0
  sub=$OUT/$c/$name; dr=$sub/ref; dc=$sub/cand
  rm -rf "$sub"; mkdir -p "$dr" "$dc"
  if [ "$name" = edgedata ]; then
    for d in "$dr" "$dc"; do
      printf '%s\n' '<additional>' "    <edgeData id=\"ed\" freq=\"60\" file=\"$d/edge.xml\"/>" '</additional>' > "$d/edgedata.add.xml"
    done
  fi
  sim=$(tr -d '\r' < "$cdir/sim.args" | sed 's/--end [0-9.]*//')
  base=(-n "$cdir/ref_simple.net.xml" -r "$cdir/routes.rou" $sim --end "$end" --no-step-log true)
  for a in "$@"; do rx+=("${a//@DIR@/$dr}"); done
  local cx=(); for a in "$@"; do cx+=("${a//@DIR@/$dc}"); done
  local t0=$SECONDS
  run_side "$REF_BIN" "$sub/ref.log" 3 "${base[@]}" "${rx[@]}" &
  local pid=$!
  run_side "$BIN" "$sub/cand.log" 1 "${base[@]}" "${cx[@]}"
  crc=$?
  wait $pid; rrc=$?
  if grep -q 'panicked at' "$sub/cand.log"; then
    verdict=PANIC; detail=$(grep -m1 -A1 'panicked at' "$sub/cand.log" | tr -d '\r' | tr '\n' ' ' | cut -c1-140)
  elif [ $crc -ne 0 ]; then
    verdict=ERROR; detail="cand exit $crc: $(grep -m1 -E 'Error|error' "$sub/cand.log" | tr -d '\r' | cut -c1-120)"
  elif [ $rrc -ne 0 ] || grep -q 'Interrupt signal' "$sub/ref.log"; then
    verdict=REFERR; detail="C++ exit $rrc: reference is not a valid oracle ($(grep -m1 -i 'interrupt' "$sub/ref.log" | cut -c1-60))"
  elif [ ! -f "$dr/$file" ] && [ ! -f "$dc/$file" ]; then
    verdict=NOOUT; detail="neither binary wrote $file"
  elif [ ! -f "$dr/$file" ] || [ ! -f "$dc/$file" ]; then
    verdict=FAIL; detail="only one binary wrote $file"
  else
    python3 tools/oracle/compare_xml.py "$dr/$file" "$dc/$file" >"$sub/compare.log" 2>&1
    case $? in
      0) verdict=PASS ;;
      1) verdict=FAIL; detail=$(head -1 "$sub/compare.log" | cut -c1-200) ;;
      *) verdict=FAIL; detail="unreadable: $(head -1 "$sub/compare.log" | cut -c1-120)" ;;
    esac
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$c" "$verdict" "$rrc" "$crc" "$detail" >> "$RESULTS"
  echo "$name $c $verdict ref_rc=$rrc cand_rc=$crc $((SECONDS-t0))s $detail"
}

for c in $CASES; do
  cdir=tools/oracle/cases/$c
  full=$(tr -d '\r' < "$cdir/sim.args" | sed -n 's/.*--end \([0-9.]*\).*/\1/p')
  full=${full:-600}
  row "$c" summary summary.xml "$full" --summary-output @DIR@/summary.xml
  row "$c" queue queue.xml "$full" --queue-output @DIR@/queue.xml
  row "$c" vehroute vehroute.xml "$full" --vehroute-output @DIR@/vehroute.xml
  row "$c" vehroute_exit vehroute_exit.xml "$full" --vehroute-output @DIR@/vehroute_exit.xml --vehroute-output.exit-times true
  row "$c" edgedata edge.xml "$full" --additional-files @DIR@/edgedata.add.xml
  row "$c" lanechange lanechange.xml "$full" --lanechange-output @DIR@/lanechange.xml
  row "$c" statistic statistic.xml "$full" --statistic-output @DIR@/statistic.xml --duration-log.statistics true
  row "$c" netstate netstate.xml 120 --netstate-dump @DIR@/netstate.xml
  row "$c" emission emission.xml 120 --emission-output @DIR@/emission.xml
  row "$c" collision collision.xml "$full" --collision-output @DIR@/collision.xml --collision.action warn --collision.mingap-factor 10
done
echo "results: $RESULTS"
