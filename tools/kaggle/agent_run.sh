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
    norm() { grep -v 'generated on\|net-file\|route-files\|additional-files\|-output value=' "$1" | sed 's/^ *//'; }
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

echo "--- official elechybrid_overhead: output checksums under repeats/allocator variants"
c=elechybrid_overhead; src="$ROOT/tools/oracle/cases/$c"; args=$(cat "$src/sim.args"); n=0
eh_variant() {
  label="$1"; shift
  n=$((n+1)); w="$OUT/ehvariant$n"; mkdir -p "$w"; cp -r "$src/." "$w/"
  a=$(printf '%s' "$args" | sed "s#tools/oracle/cases/$c#$w#g")
  [ -f "$src/extra_outputs.txt" ] && while read -r opt suf; do suf=${suf%$'\r'}; a="$a $opt $w/out_$suf.xml"; done < "$src/extra_outputs.txt"
  # shellcheck disable=SC2086
  (cd "$w" && env "$@" timeout 300 "$REF" $a --net-file "$w/ref_simple.net.xml" --route-files "$w/routes.rou" >/dev/null 2>&1)
  printf '%-24s' "$label"
  for f in elechybrid overheadwires substations; do printf '%s=%s ' "$f" "$(grep -v 'generated on\|value=' "$w/out_$f.xml" 2>/dev/null | cksum | cut -d' ' -f1)"; done; echo
}
eh_variant "default" X=1
eh_variant "default (repeat)" X=1
eh_variant "tcache_count=0" GLIBC_TUNABLES=glibc.malloc.tcache_count=0
eh_variant "top_pad=1MB" MALLOC_TOP_PAD_=1048576

echo "--- elechybrid_overhead: first rows where ref and ours differ (test1/test2/test3 vehicle lines)"
python3 - "$OUT/elechybrid_overhead/ref/out_elechybrid.xml" "$OUT/elechybrid_overhead/ours/out_elechybrid.xml" <<'PYEOF3'
import re, sys
def rows(path):
    out = []
    try:
        for line in open(path, errors="replace"):
            if "<vehicle " in line:
                d = dict(re.findall(r'(\w+)="([^"]*)"', line))
                out.append(d)
    except OSError as e:
        print("cannot read", path, e)
    return out
ref, ours = rows(sys.argv[1]), rows(sys.argv[2])
print("vehicle rows: ref=%d ours=%d" % (len(ref), len(ours)))
keys = ["current", "circuitVoltage", "alphaCircuitSolver", "power", "overheadWireId", "tractionSubstationId", "actualBatteryCapacity"]
shown = 0
for i, (a, b) in enumerate(zip(ref, ours)):
    diff = [k for k in keys if a.get(k) != b.get(k)]
    if diff:
        print("row %d id=%s pos=%s lane=%s" % (i, a.get("id"), a.get("posOnLane"), a.get("lane")))
        for k in diff:
            print("    %-22s ref=%-24s ours=%s" % (k, a.get(k), b.get(k)))
        shown += 1
        if shown >= 5:
            break
if not shown:
    print("no difference in the compared attributes")
PYEOF3

# Measure the official binary's vehicle-sized malloc/free sequence (address reuse) on md_stop.
echo "--- official md_stop: vehicle-sized malloc/free address-reuse sequence"
mkdir -p "$OUT/shim"
cat > "$OUT/shim/shim.c" <<'CEOF'
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>
static void *(*rm)(size_t);
static void (*rf)(void *);
static char boot[65536];
static size_t bo;
static int init;
#define TS 16384
static void *tab[TS];
static void ins(void *p) { size_t h = ((uintptr_t)p >> 4) % TS; while (tab[h] && tab[h] != (void *)1) h = (h + 1) % TS; tab[h] = p; }
static int del(void *p) { size_t h = ((uintptr_t)p >> 4) % TS; for (int i = 0; i < TS; i++) { if (!tab[h]) return 0; if (tab[h] == p) { tab[h] = (void *)1; return 1; } h = (h + 1) % TS; } return 0; }
static void ensure(void) { if (init) return; init = 1; rm = dlsym(RTLD_NEXT, "malloc"); rf = dlsym(RTLD_NEXT, "free"); init = 2; }
void *malloc(size_t n) {
    if (init == 1) { void *p = boot + bo; bo += (n + 15) & ~(size_t)15; return p; }
    ensure();
    void *p = rm(n);
    if (n >= 600 && n <= 3000 && p) { ins(p); char b[96]; int l = snprintf(b, sizeof b, "M %zu %p\n", n, p); if (write(2, b, l)) {} }
    return p;
}
void free(void *p) {
    if ((char *)p >= boot && (char *)p < boot + sizeof boot) return;
    if (init != 2) return;
    if (p && del(p)) { char b[64]; int l = snprintf(b, sizeof b, "F %p\n", p); if (write(2, b, l)) {} }
    rf(p);
}
CEOF
if cc -shared -fPIC -O2 -o "$OUT/shim/shim.so" "$OUT/shim/shim.c" -ldl 2>"$OUT/shim/cc.err"; then
  c=md_stop; src="$ROOT/tools/oracle/cases/$c"; w="$OUT/shimrun"; mkdir -p "$w"; cp -r "$src/." "$w/"
  a=$(sed "s#tools/oracle/cases/$c#$w#g" "$src/sim.args")
  # shellcheck disable=SC2086
  (cd "$w" && LD_PRELOAD="$OUT/shim/shim.so" timeout 300 "$REF" $a --net-file "$w/ref_simple.net.xml" --route-files "$w/routes.rou" >/dev/null 2>"$w/shim.log")
  python3 - "$w/shim.log" <<'PYEOF2'
import sys, collections
sizes = collections.Counter(); size_of = {}; ev = []
for line in open(sys.argv[1], errors="replace"):
    f = line.split()
    if len(f) == 3 and f[0] == "M":
        sizes[int(f[1])] += 1; size_of[f[2]] = int(f[1]); ev.append(("M", f[2]))
    elif len(f) == 2 and f[0] == "F":
        ev.append(("F", f[1]))
print("size histogram (size:count) top:", sizes.most_common(8))
# vehicle size: the size with 34 allocations if present, else the most common
target = [s for s, c in sizes.items() if c == 34]
vs = target[0] if target else sizes.most_common(1)[0][0]
print("assumed vehicle chunk request size:", vs)
ids = {}; out = []
for k, p in ev:
    if size_of.get(p) != vs and not (k == "M" and False):
        continue
    if k == "M":
        new = p not in ids
        ids.setdefault(p, len(ids))
        out.append(("A%d" % ids[p]) + ("" if new else "(reuse)"))
    else:
        out.append("f%d" % ids[p])
line = ""
for t in out:
    if len(line) + len(t) > 118:
        print(line); line = ""
    line += t + " "
print(line)
PYEOF2
else
  echo "shim compile failed: $(head -c 300 "$OUT/shim/cc.err")"
fi
echo "PROBE DONE (outputs in $OUT)"
