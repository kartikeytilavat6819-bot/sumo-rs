#!/bin/bash
# Random-option differential tester for netgenerate and netconvert: official SUMO 1.28.0 vs our Rust port.
# Runs on the gate VM from tools/kaggle/agent_run.sh (our binaries in $CARGO_TARGET_DIR/debug, official in
# $SUMO_PRISTINE_BIN), or anywhere both exist.
#   fuzz_net.sh [N=24]     run seeds FIRST..FIRST+N-1, one status line per stage (gen/conv) with the first diff;
#                          then print a FAILED block with the exact args + diffs for each distinct signature.
# Stage 1: both netgenerates on identical random args; net compared with compare_net.py.
# Stage 2: both netconverts re-importing the *official* net from stage 1 with random processing options.
# Env: FUZZ_OUT (work dir), FUZZ_FIRST (first seed), FUZZ_BUDGET (s, default 900).
set -u
cd "$(dirname "$0")/../.." || exit 1
ORA=tools/oracle
OFF=${SUMO_PRISTINE_BIN:?}; OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug
PY=$(command -v python3 || command -v python)
OUT=${FUZZ_OUT:-/tmp/fuzz_net}; mkdir -p "$OUT"
N=${1:-24}; FIRST=${FUZZ_FIRST:-1}; BUDGET=${FUZZ_BUDGET:-900}
START=$(date +%s)
r() { R=$(( RANDOM % $1 )); }   # bash RANDOM advances only in this shell (no subshell)

gen_case() {  # gen_case <seed> <dir>; writes netgen.args, conv.args; prints a short summary
  local s=$1 d=$2 mode lanes jt ng cv o
  mkdir -p "$d"; RANDOM=$s; r 1; r 1; r 1
  r 4; local modes=(grid grid spider rand); mode=${modes[$R]}
  r 3; lanes=$((R + 1))
  r 5; local jts=(priority traffic_light right_before_left allway_stop priority); jt=${jts[$R]}
  ng="--seed $s --default.lanenumber $lanes --default-junction-type $jt"
  case $mode in
    grid)   r 3; ng="--grid --grid.number $((R + 2)) --grid.length $((80 + R * 20)) $ng";;
    spider) r 3; ng="--spider --spider.arm-number $((R + 3)) --spider.circle-number 2 --spider.space-radius 100 $ng";;
    rand)   r 3; ng="--rand --rand.iterations $((10 + R * 5)) --rand.max-distance 300 --rand.grid true $ng";;
  esac
  local pool=("--no-turnarounds true" "--junctions.corner-detail 3" "--sidewalks.guess true"
    "--crossings.guess true" "--bikelanes.guess true" "--roundabouts.guess true"
    "--ramps.guess true" "--geometry.remove true" "--edges.join true"
    "--no-internal-links true" "--lefthand true" "--tls.guess true")
  local picked=""
  for o in "${pool[@]}"; do r 3; [ $R -eq 0 ] && picked="$picked $o"; done
  ng="$ng$picked"
  printf '%s\n' "$ng" > "$d/netgen.args"

  local pool2=("--no-turnarounds true" "--junctions.corner-detail 4" "--sidewalks.guess true"
    "--crossings.guess true" "--bikelanes.guess true" "--roundabouts.guess true"
    "--ramps.guess true" "--geometry.remove true" "--edges.join true"
    "--no-internal-links true" "--lefthand true" "--tls.guess true"
    "--tls.guess.joining true" "--opposites.guess true" "--junctions.join true"
    "--junctions.limit-turn-speed 5.5" "--speed.factor 0.8" "--default.speed 8.33"
    "--keep-edges.components 1" "--tls.discard-simple true" "--tls.layout incoming")
  cv=""
  for o in "${pool2[@]}"; do r 4; [ $R -eq 0 ] && cv="$cv $o"; done
  printf '%s\n' "$cv" > "$d/conv.args"
  echo "mode=$mode lanes=$lanes jt=$jt ng=[$picked] cv=[$cv]"
}

cmp_net() {  # cmp_net <ref> <cand> <out.diff>; DIFF set; 0 same, 1 differ, 2 parse error
  "$PY" "$ORA/compare_net.py" "$1" "$2" > "$3" 2>&1
  local rc=$?
  if [ $rc -eq 0 ]; then DIFF=""; return 0; fi
  if [ $rc -eq 2 ]; then DIFF="PARSE $(head -1 "$3" | cut -c1-140)"; return 1; fi
  DIFF="$(head -5 "$3" | tr '\n' ' ' | cut -c1-480)"; return 1
}
exitof() {  # exitof <log>: first error-ish line
  grep -m1 -iE 'error|exception' "$1" | cut -c1-140
}
run_gen() {  # run_gen <dir>: DIFF set; 0 pass, 1 fail, 2 panic, 3 skip
  local d=$1 a rr cr
  [ -x "$OURS/netgenerate" ] || { DIFF="SKIP: our netgenerate binary missing"; return 3; }
  a=$(cat "$d/netgen.args")
  timeout 90 "$OFF/netgenerate" $a -o "$d/cpp.net.xml" >"$d/cpp.gen.log" 2>&1; cr=$?
  timeout 90 "$OURS/netgenerate" $a -o "$d/rust.net.xml" >"$d/rust.gen.log" 2>&1; rr=$?
  if grep -q panicked "$d/rust.gen.log"; then
    DIFF="PANIC $(grep -m1 -A1 panicked "$d/rust.gen.log" | tr '\n' ' ' | cut -c1-200)"; return 2; fi
  if [ $cr -ne $rr ]; then DIFF="exit rust=$rr cpp=$cr rust:$(exitof "$d/rust.gen.log") cpp:$(exitof "$d/cpp.gen.log")"; return 1; fi
  if [ $cr -ne 0 ]; then DIFF="SKIP: both exit $cr cpp:$(exitof "$d/cpp.gen.log")"; return 3; fi
  cmp_net "$d/cpp.net.xml" "$d/rust.net.xml" "$d/gen.diff"; return $?
}
run_conv() {  # run_conv <dir>: DIFF set; 0 pass, 1 fail, 2 panic, 3 skip
  local d=$1 a rr cr
  [ -s "$d/cpp.net.xml" ] || { DIFF="SKIP: no official net"; return 3; }
  a=$(cat "$d/conv.args")
  timeout 90 "$OFF/netconvert" -s "$d/cpp.net.xml" $a -o "$d/cpp.conv.net.xml" >"$d/cpp.conv.log" 2>&1; cr=$?
  timeout 90 "$OURS/netconvert" -s "$d/cpp.net.xml" $a -o "$d/rust.conv.net.xml" >"$d/rust.conv.log" 2>&1; rr=$?
  if grep -q panicked "$d/rust.conv.log"; then
    DIFF="PANIC $(grep -m1 -A1 panicked "$d/rust.conv.log" | tr '\n' ' ' | cut -c1-200)"; return 2; fi
  if [ $cr -ne $rr ]; then DIFF="exit rust=$rr cpp=$cr rust:$(exitof "$d/rust.conv.log") cpp:$(exitof "$d/cpp.conv.log")"; return 1; fi
  if [ $cr -ne 0 ]; then DIFF="SKIP: both exit $cr cpp:$(exitof "$d/cpp.conv.log")"; return 3; fi
  cmp_net "$d/cpp.conv.net.xml" "$d/rust.conv.net.xml" "$d/conv.diff"; return $?
}
sig() { echo "$1" | sed -E "s/'[^']*'/'X'/g; s/[0-9]+(\.[0-9]+)?/N/g" | cut -c1-110; }

set +u; declare -A SEEN; FAILS=""; P=0; F=0; S=0
for ((s = FIRST; s < FIRST + N; s++)); do
  [ $(( $(date +%s) - START )) -gt $BUDGET ] && { echo "budget: stopped at seed $s"; break; }
  d=$OUT/seed$s; rm -rf "$d"
  params=$(gen_case $s "$d")
  for stage in gen conv; do
    if [ $stage = gen ]; then run_gen "$d"; else run_conv "$d"; fi
    rc=$?
    case $rc in
      0) st=PASS; P=$((P+1));;
      1) st=FAIL; F=$((F+1));;
      2) st=PANIC; F=$((F+1));;
      3) st=SKIP; S=$((S+1));;
    esac
    echo "seed $s $stage $st ${DIFF:+| $DIFF}" | cut -c1-620
    if [ $rc -eq 1 ] || [ $rc -eq 2 ]; then
      k="$stage|$(sig "$DIFF")"
      if [ -z "${SEEN[$k]:-}" ]; then SEEN[$k]=$s; FAILS="$FAILS $s:$stage"; fi
    fi
  done
  echo "  params seed $s: $params"
done
echo "=== PASS=$P FAIL=$F SKIP=$S; distinct signatures: ${#SEEN[@]}"
echo "=== FAILED details (first seed per signature):"
for f in $FAILS; do
  sd=${f%%:*}; stage=${f##*:}; d=$OUT/seed$sd
  echo "--- seed $sd $stage: netgen.args=[$(cat "$d/netgen.args")] conv.args=[$(cat "$d/conv.args")]"
  if [ $stage = gen ]; then head -6 "$d/gen.diff" 2>/dev/null; grep -m1 -A1 panicked "$d/rust.gen.log" 2>/dev/null;
  else head -6 "$d/conv.diff" 2>/dev/null; grep -m1 -A1 panicked "$d/rust.conv.log" 2>/dev/null; fi
done
