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
    # extra_outputs.txt lines are "<option> <suffix>"; compared against ref.<suffix>.xml
    [ -f "$src/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; a="$a $opt $w/out_$suf.xml"; done < "$src/extra_outputs.txt"
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
for c in md_midinterval md_stop; do
  f=out_ed_mid.xml; [ $c = md_stop ] && f=out_ed_stop_tr.xml
  for side in ref ours; do
    echo "--- $c $side: interval headers + edge -103224#0/#1 rows"
    grep -h '<interval\|id="-103224#0"\|id="-103224#1"' "$OUT/$c/$side/$f" | sed 's/^ *//' | head -n 30
  done
done
# Is the official tracker output allocator dependent? (stale vehicle-pointer keys in myTrackedData)
echo "--- official md_stop ed_stop_tr edge -103224#0 per-interval sampledSeconds under allocator variants"
c=md_stop; src="$ROOT/tools/oracle/cases/$c"; args=$(cat "$src/sim.args"); n=0
run_variant() { # $1 = label, rest = env assignments
  label="$1"; shift
  n=$((n+1)); w="$OUT/variant$n"; mkdir -p "$w"; cp -r "$src/." "$w/"
  a=$(printf '%s' "$args" | sed "s#tools/oracle/cases/$c#$w#g")
  # shellcheck disable=SC2086
  (cd "$w" && env "$@" timeout 300 "$REF" $a --net-file "$w/ref_simple.net.xml" --route-files "$w/routes.rou" >/dev/null 2>&1)
  printf '%-34s' "$label"
  grep -h 'id="-103224#0"' "$w/out_ed_stop_tr.xml" | sed 's/.*sampledSeconds="\([0-9.]*\)".*/\1/' | tr '\n' ' '; echo
}
run_variant "default" X=1
run_variant "default (repeat)" X=1
run_variant "env pad 4000" PAD="$(head -c 4000 /dev/zero | tr '\0' x)"
run_variant "tcache_count=0" GLIBC_TUNABLES=glibc.malloc.tcache_count=0
run_variant "tcache_count=1" GLIBC_TUNABLES=glibc.malloc.tcache_count=1
run_variant "mmap_threshold=65536" MALLOC_MMAP_THRESHOLD_=65536
run_variant "top_pad=1MB" MALLOC_TOP_PAD_=1048576
run_variant "arena_max=1" MALLOC_ARENA_MAX=1
echo "--- official out_bt: number of <seen> entries and checksum under allocator variants"
c=out_bt; src="$ROOT/tools/oracle/cases/$c"; args=$(cat "$src/sim.args"); n=0
bt_variant() {
  label="$1"; shift
  n=$((n+1)); w="$OUT/btvariant$n"; mkdir -p "$w"; cp -r "$src/." "$w/"
  a=$(printf '%s' "$args" | sed "s#tools/oracle/cases/$c#$w#g")
  # shellcheck disable=SC2086
  (cd "$w" && env "$@" timeout 300 "$REF" $a --net-file "$w/ref_simple.net.xml" --route-files "$w/routes.rou" --bt-output "$w/out_bt.xml" >/dev/null 2>&1)
  printf '%-30s seen=%s bt=%s sum=%s\n' "$label" "$(grep -c '<seen' "$w/out_bt.xml" 2>/dev/null)" "$(grep -c '<bt ' "$w/out_bt.xml" 2>/dev/null)" "$(grep -v 'generated on\|value=' "$w/out_bt.xml" 2>/dev/null | cksum | cut -d' ' -f1)"
}
bt_variant "default" X=1
bt_variant "default (repeat)" X=1
bt_variant "tcache_count=0" GLIBC_TUNABLES=glibc.malloc.tcache_count=0
bt_variant "top_pad=1MB" MALLOC_TOP_PAD_=1048576
echo "PROBE DONE (outputs in $OUT)"
