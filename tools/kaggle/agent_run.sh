#!/usr/bin/env bash
# oc95 probe: run the official binary and ours on the target cases, print first differences.
# Runs on the gate VM from the repo root. Writes only under /tmp/oc95.
set -u
ID=oc95
OUT=/tmp/$ID
rm -rf "$OUT"; mkdir -p "$OUT"
OURS="${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo"
REF="${SUMO_PRISTINE_BIN:?}/sumo"
export SUMO_HOME="${SUMO_HOME:-}"
CASES="md_midinterval md_stop md_vtypes out_meandata out_bt elechybrid_overhead"
ROOT=$(pwd)
export OC95_TRACE=1

for c in $CASES; do
  src="$ROOT/tools/oracle/cases/$c"
  [ -d "$src" ] || { echo "$c: case dir missing"; continue; }
  args=$(cat "$src/sim.args")
  for side in ref ours; do
    w="$OUT/$c/$side"
    mkdir -p "$w"
    cp -r "$src/." "$w/"
    # sim.args uses repo-relative case paths; point them at the private copy
    a=$(printf '%s' "$args" | sed "s#tools/oracle/cases/$c#$w#g")
    bin="$REF"; [ "$side" = ours ] && bin="$OURS"
    # shellcheck disable=SC2086
    (cd "$w" && timeout 900 "$bin" $a \
        --net-file "$w/ref_simple.net.xml" --route-files "$w/routes.rou" \
        > "$w/stdout.txt" 2> "$w/stderr.txt")
    echo "$c $side exit=$?"
    head -n 3 "$w/stderr.txt" | sed "s/^/  stderr: /"
  done
  # compare every reference output we have against the same-named produced file
  for r in "$src"/ref.*.xml; do
    [ -f "$r" ] || continue
    s=$(basename "$r" | sed 's/^ref\.//')
    got=""
    for cand in "$OUT/$c/ours/out_${s}" "$OUT/$c/ours/${s}"; do
      [ -f "$cand" ] && { got="$cand"; break; }
    done
    case "$s" in fcd*|stat*|tripinfo*) continue ;; esac
    if [ -z "$got" ]; then echo "$c/$s: no output produced"; continue; fi
    refout="$OUT/$c/ref/out_${s}"; [ -f "$refout" ] || refout="$OUT/$c/ref/${s}"
    norm() { grep -v 'generated on\|net-file\|route-files\|additional-files' "$1" | sed 's/^ *//'; }
    if diff -q <(norm "$refout") <(norm "$got") >/dev/null; then
      echo "$c/$s: identical"
    else
      echo "$c/$s: DIFF (ref < > ours), first differing lines:"
      diff <(norm "$refout") <(norm "$got") | head -n 10 | sed 's/^/  /'
    fi
  done
done
echo "--- OC95T trace (md_midinterval ours, first 40 lines)"
grep -h "OC95T leave" "$OUT/md_midinterval/ours/stderr.txt" | awk '{print $2}' | sort | uniq -c
echo "--- OC95T enter counts per tracker"
grep -h "OC95T enter" "$OUT/md_midinterval/ours/stderr.txt" | awk '{print $2}' | sort | uniq -c
echo "PROBE DONE (outputs in $OUT)"
