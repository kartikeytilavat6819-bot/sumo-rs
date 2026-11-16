#!/bin/bash
# Output-file oracle: for each output option, run C++ sumo (WSL) and our sumo.exe on the same case with
# the case's sim.args, then compare the produced XML element by element (compare_xml.py).
# Usage: bash tools/oracle/run_outputs.sh [case ...]     default: real signal stress lanechange
# Writes $OUT/<case>/<output>/{ref,cand}/ and $OUT/results.tsv (truncated each run). Never writes into cases/.
# BIN selects our binary (default: the x07 debug build, not rebuilt here).
set -u
cd "$(dirname "$0")/../.." || exit 1
ROOT=$(pwd)
W=/mnt/c${ROOT#/c}
OUT=${OUT:-/c/Users/karti/Temp/score/outputs}
BIN=${BIN:-/c/Users/karti/sumo-rs-target-x07/debug/sumo.exe}
RESULTS=$OUT/results.tsv
mkdir -p "$OUT"
: > "$RESULTS"
printf 'output\tcase\tverdict\tref_rc\tcand_rc\tdetail\n' >> "$RESULTS"

# row <case> <output name> <file written> <extra args...>; @DIR@ in the args is the per-side output dir.
row() {
  local c=$1 name=$2 file=$3; shift 3
  local extra="$*"
  local sub=$OUT/$c/$name dr dc
  dr=$sub/ref; dc=$sub/cand
  rm -rf "$sub"; mkdir -p "$dr" "$dc"
  if [ "$name" = edgedata ]; then
    for d in "$dr" "$dc"; do
      printf '%s\n' '<additional>' '    <edgeData id="ed" freq="60" file="edge.xml"/>' '</additional>' > "$d/edgedata.add.xml"
    done
  fi
  local sim base
  sim=$(cat "cases/$c/sim.args" 2>/dev/null)
  base="-n tools/oracle/cases/$c/ref_simple.net.xml -r tools/oracle/cases/$c/routes.rou $sim"
  local rargs=${extra//@DIR@//mnt/c${dr#/c}}
  local cargs=${extra//@DIR@/$dc}
  # C++ sometimes exits 15 (SIGTERM) or gets "Interrupt signal received" with a truncated file (seen
  # 2026-11-16, not reproducible on demand): retry up to 3 times, keep the last attempt.
  local rrc=0 try
  for try in 1 2 3; do
    wsl -e bash -lc "cd $W && SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/sumo $base $rargs --no-step-log true" \
        </dev/null >"$sub/ref.log" 2>&1
    rrc=$?
    [ $rrc -eq 0 ] && ! grep -q 'Interrupt signal' "$sub/ref.log" && break
  done
  [ "$try" -gt 1 ] && echo "  (C++ needed $try attempts for $name $c)"
  "$BIN" $base $cargs --no-step-log true >"$sub/cand.log" 2>&1
  local crc=$?
  local verdict detail=""
  if grep -q 'panicked at' "$sub/cand.log"; then
    verdict=PANIC; detail=$(grep -m1 -A1 'panicked at' "$sub/cand.log" | tr -d '\r' | tr '\n' ' ' | cut -c1-140)
  elif [ $crc -ne 0 ]; then
    verdict=ERROR; detail="cand exit $crc: $(grep -m1 -E 'Error|error' "$sub/cand.log" | tr -d '\r' | cut -c1-120)"
  elif [ $rrc -ne 0 ] || grep -q 'Interrupt signal' "$sub/ref.log"; then
    verdict=REFERR; detail="C++ exit $rrc: reference is not a valid oracle, rerun ($(grep -m1 -i 'interrupt' "$sub/ref.log" | cut -c1-60))"
  elif [ ! -f "$dr/$file" ] && [ ! -f "$dc/$file" ]; then
    verdict=NOOUT; detail="neither binary wrote $file"
  elif [ ! -f "$dr/$file" ] || [ ! -f "$dc/$file" ]; then
    verdict=FAIL; detail="only one binary wrote $file"
  else
    python tools/oracle/compare_xml.py "$dr/$file" "$dc/$file" >"$sub/compare.log" 2>&1
    case $? in
      0) verdict=PASS ;;
      1) verdict=FAIL; detail=$(head -1 "$sub/compare.log" | cut -c1-200) ;;
      *) verdict=FAIL; detail="unreadable: $(head -1 "$sub/compare.log" | cut -c1-120)" ;;
    esac
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$c" "$verdict" "$rrc" "$crc" "$detail" >> "$RESULTS"
  echo "$name $c $verdict ref_rc=$rrc cand_rc=$crc $detail"
}

CASES=${*:-real signal stress lanechange}
for c in $CASES; do
  if [ "$c" = lanechange ]; then
    row "$c" lanechange lanechange.xml --lanechange-output @DIR@/lanechange.xml
    continue
  fi
  row "$c" summary summary.xml --summary-output @DIR@/summary.xml
  row "$c" queue queue.xml --queue-output @DIR@/queue.xml
  row "$c" netstate netstate.xml --netstate-dump @DIR@/netstate.xml
  row "$c" vehroute vehroute.xml --vehroute-output @DIR@/vehroute.xml
  row "$c" vehroute_exit vehroute.xml --vehroute-output @DIR@/vehroute.xml --vehroute-output.exit-times true
  row "$c" edgedata edge.xml --additional-files @DIR@/edgedata.add.xml
  row "$c" full full.xml --full-output @DIR@/full.xml
  row "$c" amitran amitran.xml --amitran-output @DIR@/amitran.xml
  row "$c" link link.xml --link-output @DIR@/link.xml
  row "$c" person person.xml --person-summary-output @DIR@/person.xml
  if [ "$c" = stress ]; then
    row "$c" collision collision.xml --collision-output @DIR@/collision.xml --collision.action warn
  fi
done
echo "results: $RESULTS"
