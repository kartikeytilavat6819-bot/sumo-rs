#!/bin/bash
# Gate diagnostic for oc55: full tripinfo diff + stuck-vehicle detection.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo"
[ -x "$SUMO" ] || SUMO="$BIN/sumo.exe"
OUT="${TMPDIR:-/tmp}/oc55-spot"
rm -rf "$OUT"; mkdir -p "$OUT" || { echo "no temp dir $OUT"; exit 1; }
c=tt_extended_collisions_intermodal_stoptime_remove
d=tools/oracle/cases/$c
flags=$(cat "$d/sim.args")
timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
    --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
    --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
echo "rc=$?"
echo "=== full tripinfo compare ==="
python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" 2>&1 | head -60
echo "=== vehicles in ref vs cand tripinfo ==="
python - "$d/ref.tripinfo.xml" "$OUT/ti.xml" <<'PY'
import re,sys
def ids(p): return set(re.findall(r'<tripinfo id="([^"]+)"',open(p).read()))
r,c=ids(sys.argv[1]),ids(sys.argv[2])
print("ref-only:",sorted(r-c))
print("cand-only:",sorted(c-r))
PY
echo "=== fcd vehicle counts at t=85 ==="
python - "$d/ref.fcd.xml" "$OUT/fcd.xml" <<'PY'
import re,sys
def at85(p):
    s=open(p).read().split('timestep time="85.00"')
    return set(re.findall(r'<(?:vehicle|person) id="([^"]+)"',s[1].split('</timestep>')[0])) if len(s)>1 else set()
r,c=at85(sys.argv[1]),at85(sys.argv[2])
print("ref at85:",sorted(r))
print("cand at85:",sorted(c))
PY
