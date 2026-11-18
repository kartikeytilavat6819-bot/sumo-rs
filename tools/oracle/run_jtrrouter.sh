#!/bin/bash
# Route oracle for jtrrouter: runs the official SUMO 1.28.0 jtrrouter and ours on each case and
# compares the written route files byte for byte. The reference is always run live from the
# pristine binary (never committed): $SUMO_PRISTINE_BIN/jtrrouter on the VM, ~/sumo-pristine/bin in WSL.
# A case is a directory cases/jtrrouter_<name>/ with
#   net   one line: the network file (repo-relative)
#   args  one line: jtrrouter options; @DIR@ is replaced by the per-side output dir, @CASE@ by the case dir
# Usage: bash tools/oracle/run_jtrrouter.sh [case ...]     default: every cases/jtrrouter_*
# BIN selects our binary (default $CARGO_TARGET_DIR/debug/jtrrouter, /tmp/target on Kaggle).
# Writes $OUT/<case>/{ref,cand}/ and $OUT/results.tsv (truncated each run).
set -u
cd "$(dirname "$0")/../.." || exit 1
if [ -n "${SUMO_PRISTINE_BIN:-}" ]; then
  REF_BIN=${REF_BIN:-$SUMO_PRISTINE_BIN/jtrrouter}
  export SUMO_HOME=${SUMO_HOME:-$SUMO_PRISTINE_BIN/../lib/python3.12/site-packages/sumo}
else
  REF_BIN=${REF_BIN:-$HOME/sumo-pristine/bin/jtrrouter}
  export SUMO_HOME=${SUMO_HOME:-$HOME/sumo-pristine/lib/python3.12/site-packages/sumo}
fi
BIN=${BIN:-${CARGO_TARGET_DIR:-/tmp/target}/debug/jtrrouter}
OUT=${OUT:-${TMPDIR:-/tmp}/jtrrouter_oracle}
RESULTS=$OUT/results.tsv
mkdir -p "$OUT"
: > "$RESULTS"
printf 'case\tverdict\tref_rc\tcand_rc\tdetail\n' >> "$RESULTS"

# the generation timestamp differs between runs by design, and the embedded configuration names the per-side
# output dir (ref/ or cand/); every other byte must match
normalize() { grep -v 'generated on' "$1" | sed -E 's#/(ref|cand)/#/SIDE/#g'; }

run_case() {
  local c=$1 cdir=tools/oracle/cases/$1
  local sub=$OUT/$c net args rargs cargs rrc crc verdict detail="" f
  rm -rf "$sub"; mkdir -p "$sub/ref" "$sub/cand"
  net=$(tr -d '' < "$cdir/net")
  args=$(tr -d '' < "$cdir/args")
  rargs=${args//@CASE@/$cdir}; rargs=${rargs//@DIR@/$sub/ref}
  cargs=${args//@CASE@/$cdir}; cargs=${cargs//@DIR@/$sub/cand}
  "$REF_BIN" -n "$net" $rargs --no-step-log true >"$sub/ref.log" 2>&1
  rrc=$?
  "$BIN" -n "$net" $cargs --no-step-log true >"$sub/cand.log" 2>&1
  crc=$?
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

CASES=${*:-$(cd tools/oracle/cases && ls -d jtrrouter_* | tr '\n' ' ')}
for c in $CASES; do
  run_case "$c"
done
echo "results: $RESULTS"
