#!/bin/bash
# State save/load parity for every sim case and sim_<variant>.args.
# Per variant: ours and the official C++ build save their state at the middle of the run (--save-state.times),
# the state bodies must be identical (compare_state.py), then each side loads its own state (--load-state) and
# the continued tripinfo of ours must match the C++ one (compare_sim.py --tripinfo).
# Usage: bash tools/oracle/run_state.sh [case ...]      (default: every case; JOBS=n parallel cases, default 8)
set -u
cd "$(dirname "$0")/../.." || exit 1
ROOT=$(pwd); export ROOT
BIN="${CARGO_TARGET_DIR:-/tmp/target}/debug"
OURS=$BIN/sumo.exe
PRISTINE=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}/sumo
export OURS PRISTINE
[ -e "$OURS" ] || { echo "MISSING BINARY: $OURS (build first)"; exit 1; }
[ -e "$PRISTINE" ] || { echo "MISSING PRISTINE: $PRISTINE"; exit 1; }
OUT=${STATE_OUT:-${SCORE_OUT:-/tmp/score}/state-$$}
mkdir -p "$OUT"; export OUT

# one sim run; <ours|pristine> <log prefix> <args...>
sumo_run() {
  local bin=$OURS
  [ "$1" = pristine ] && bin=$PRISTINE
  "$bin" -n "$NET" -r "$ROU" --no-step-log true "${@:3}" >"$2.log" 2>&1
}

# one sim case: prints one line per variant
run_case() {
  local c=$1 dir=$ROOT/tools/oracle/cases/$1 o=$OUT/$1 a tag v EXTRA end mid st ld veh why
  mkdir -p "$o"
  NET=$dir/ref_simple.net.xml; ROU=$dir/routes.rou
  for a in "$dir"/sim.args "$dir"/sim_*.args; do
    [ -f "$a" ] || continue
    tag=$(basename "$a" .args); v=$o/$tag; EXTRA=$(cat "$a")
    end=$(echo "$EXTRA" | sed -n 's/.*--end \([0-9.]*\).*/\1/p')
    mid=$(awk -v e="${end:-100}" 'BEGIN{print e/2}')
    sumo_run ours "$v.save_ours" $EXTRA --save-state.times "$mid" --save-state.files "$v.ours.state.xml"
    sumo_run pristine "$v.save_pristine" $EXTRA --save-state.times "$mid" --save-state.files "$v.pristine.state.xml"
    if [ ! -f "$v.ours.state.xml" ] && [ ! -f "$v.pristine.state.xml" ]; then
      # short routes end before mid: both sides save at whole seconds, half of the last arrival (a save fires only on a step boundary)
      sumo_run pristine "$v.full" $EXTRA --tripinfo-output "$v.full.ti.xml"
      last=$(grep -o ' arrival="[0-9.]*"' "$v.full.ti.xml" 2>/dev/null | sed 's/[^0-9.]//g' | sort -g | tail -1)
      mid=$(awk -v e="${last:-0}" 'BEGIN{ if (e >= 2) print int(e / 2); else print "" }')
      if [ -n "$mid" ]; then
        sumo_run ours "$v.save_ours" $EXTRA --save-state.times "$mid" --save-state.files "$v.ours.state.xml"
        sumo_run pristine "$v.save_pristine" $EXTRA --save-state.times "$mid" --save-state.files "$v.pristine.state.xml"
      fi
    fi
    st=PASS; ld=SKIP; why=""
    if [ ! -f "$v.ours.state.xml" ] && [ ! -f "$v.pristine.state.xml" ]; then
      st=NONE; why=$(grep -m1 -hE 'panicked at|Error' "$v.save_ours.log" "$v.save_pristine.log" 2>/dev/null | cut -c1-90)
    elif [ ! -f "$v.ours.state.xml" ] || [ ! -f "$v.pristine.state.xml" ]; then
      st=NOSTATE
      why=$(grep -m1 -hE 'panicked at|Error' "$v.save_ours.log" "$v.save_pristine.log" 2>/dev/null | cut -c1-90)
    else
      python3 tools/oracle/compare_state.py "$v.ours.state.xml" "$v.pristine.state.xml" >"$v.state.txt" 2>&1 || st=FAIL
      [ $st = FAIL ] && why=$(grep -E '^[-+][^-+]' "$v.state.txt" | head -2 | tr '\n' ' ' | cut -c1-200)
    fi
    veh=$(grep -c '<vehicle ' "$v.ours.state.xml" 2>/dev/null)
    if [ "$st" = PASS ] || [ "$st" = FAIL ]; then
      sumo_run ours "$v.load_ours" $EXTRA --load-state "$v.ours.state.xml" --tripinfo-output "$v.load_ours.ti.xml"
      sumo_run pristine "$v.load_pristine" $EXTRA --load-state "$v.pristine.state.xml" --tripinfo-output "$v.load_pristine.ti.xml"
      if [ -f "$v.load_ours.ti.xml" ] && [ -f "$v.load_pristine.ti.xml" ]; then
        ld=PASS
        python3 tools/oracle/compare_sim.py --tripinfo "$v.load_pristine.ti.xml" "$v.load_ours.ti.xml" >"$v.load.txt" 2>&1 || ld=FAIL
        [ $ld = FAIL ] && why="load: $(head -1 "$v.load.txt" | cut -c1-90)"
      else
        ld=NOLOAD; why=$(grep -m1 -hE 'panicked at|Error' "$v.load_ours.log" "$v.load_pristine.log" 2>/dev/null | cut -c1-90)
      fi
    fi
    rm -f "$dir/out_edgedata.xml"
    printf '%-34s mid=%-6s veh=%-4s state=%-7s load=%-7s %s\n' "$c/$tag" "$mid" "${veh:-0}" "$st" "$ld" "$why"
    [ "$st" = FAIL ] && grep -E '^[-+][^-+]' "$v.state.txt" | head -8 | cut -c1-260 | sed 's/^/      state /'
    [ "$ld" = FAIL ] && head -3 "$v.load.txt" | cut -c1-200 | sed 's/^/      load /'
  done
}

if [ "${1:-}" = --one ]; then run_case "$2"; exit 0; fi

if [ $# -gt 0 ]; then cases=("$@")
else cases=($(ls -d tools/oracle/cases/*/ | xargs -n1 basename)); fi
export -f run_case sumo_run

printf '%s\n' "${cases[@]}" | xargs -P "${JOBS:-8}" -I{} bash "$0" --one {} | tee "$OUT/results.txt"
echo "-------------------------------------------"
echo "variants=$(grep -c ' state=' "$OUT/results.txt") state_PASS=$(grep -c 'state=PASS' "$OUT/results.txt") load_PASS=$(grep -c 'load=PASS' "$OUT/results.txt")"
grep -E 'state=(FAIL|NOSTATE)|load=(FAIL|NOLOAD)|SKIP' "$OUT/results.txt"
echo "STATE DONE"
