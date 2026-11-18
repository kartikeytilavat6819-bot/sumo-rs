#!/bin/bash
# Oracle for dfrouter / activitygen: runs the official 1.28.0 tool and ours on each case and compares every written
# file byte for byte (except the generation timestamp; the per-side dir name is normalized).
# A case is cases/<tool>_<name>/args (one line; @DIR@ = per-side output dir). Reference is run live from $SUMO_PRISTINE_BIN.
# Usage: TOOL=dfrouter|activitygen bash tools/oracle/run_dfag.sh [case ...]   Writes $OUT/results.tsv
set -u
cd "$(dirname "$0")/../.." || exit 1
TOOL=${TOOL:?TOOL=dfrouter|activitygen}
if [ -n "${SUMO_PRISTINE_BIN:-}" ]; then
  REF_BIN=${REF_BIN:-$SUMO_PRISTINE_BIN/$TOOL}
  export SUMO_HOME=${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}
else
  REF_BIN=${REF_BIN:-$HOME/sumo-pristine/bin/$TOOL}
  export SUMO_HOME=${SUMO_HOME:-$HOME/sumo-pristine/lib/python3.12/site-packages/sumo}
fi
BIN=${BIN:-${CARGO_TARGET_DIR:-/tmp/target}/debug/$TOOL}
OUT=${OUT:-${TMPDIR:-/tmp}/${TOOL}_oracle}
RESULTS=$OUT/results.tsv
mkdir -p "$OUT"; : > "$RESULTS"
printf 'case\tverdict\tref_rc\tcand_rc\tdetail\n' >> "$RESULTS"
normalize() { grep -v 'generated on' "$1" | sed -E 's#/(ref|cand)/#/SIDE/#g'; }
run_case() {
  local c=$1 cdir=tools/oracle/cases/$1
  local sub=$OUT/$c args rargs cargs rrc crc verdict detail="" f
  rm -rf "$sub"; mkdir -p "$sub/ref" "$sub/cand"
  args=$(cat "$cdir/args")
  rargs=${args//@DIR@/$sub/ref}; cargs=${args//@DIR@/$sub/cand}
  "$REF_BIN" $rargs >"$sub/ref.log" 2>&1; rrc=$?
  "$BIN" $cargs >"$sub/cand.log" 2>&1; crc=$?
  if grep -q 'panicked at' "$sub/cand.log"; then
    verdict=PANIC; detail=$(grep -m1 -A1 'panicked at' "$sub/cand.log" | tr -d '\r' | tr '\n' ' ' | cut -c1-160)
  elif [ $crc -ne 0 ]; then
    verdict=ERROR; detail="cand exit $crc: $(grep -m1 -iE 'error' "$sub/cand.log" | tr -d '\r' | cut -c1-140)"
  elif [ $rrc -ne 0 ]; then
    verdict=REFERR; detail="ref exit $rrc: $(grep -m1 -iE 'error' "$sub/ref.log" | tr -d '\r' | cut -c1-140)"
  elif [ "$(cd "$sub/ref" && ls)" != "$(cd "$sub/cand" && ls)" ]; then
    verdict=FAIL; detail="written files differ: ref [$(ls "$sub/ref" | tr '\n' ' ')] cand [$(ls "$sub/cand" | tr '\n' ' ')]"
  else
    verdict=PASS
    for f in $(ls "$sub/ref"); do
      if ! diff <(normalize "$sub/ref/$f") <(normalize "$sub/cand/$f") >"$sub/$f.diff" 2>&1; then
        verdict=FAIL; detail="$f: $(head -1 "$sub/$f.diff" | cut -c1-120)"; break
      fi
    done
  fi
  printf '%s\t%s\t%s\t%s\t%s\n' "$c" "$verdict" "$rrc" "$crc" "$detail" >> "$RESULTS"
  echo "$c $verdict ref_rc=$rrc cand_rc=$crc $detail"
}
CASES=${*:-$(cd tools/oracle/cases && ls -d ${TOOL}_* | grep -v _data | tr '\n' ' ')}
for c in $CASES; do run_case "$c"; done
echo "results: $RESULTS"
