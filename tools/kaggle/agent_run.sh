#!/bin/bash
# Gate diagnostic for oc55: dump candidate vs ref trajectories for the failing collisions rows.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo"
[ -x "$SUMO" ] || SUMO="$BIN/sumo.exe"
OUT="${TMPDIR:-/tmp}/oc55-spot"
rm -rf "$OUT"; mkdir -p "$OUT" || { echo "no temp dir $OUT"; exit 1; }
for c in tt_extended_collisions_intermodal_stoptime_remove tt_extended_collisions_intermodal_stoptime_warn; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args")
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  echo "===== $c rc=$? ====="
  grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" && continue
  python - "$d/ref.fcd.xml" "$OUT/fcd.xml" <<'PY'
import re,sys
def traj(path):
    t=None; out={}
    for line in open(path):
        m=re.search(r'timestep time="([\d.]+)"',line)
        if m: t=float(m.group(1))
        m=re.search(r'vehicle id="(WE\.1|NE\.3)"[^>]*x="([\d.]+)"[^>]*y="[\d.]+"[^>]*angle="[\d.]+"[^>]*type="[^"]+"[^>]*speed="([\d.]+)"',line)
        if m and 45<=t<=85: out.setdefault(m.group(1),[]).append((t,m.group(2),m.group(3)))
    return out
r=traj(sys.argv[1]); c=traj(sys.argv[2])
for veh in ("WE.1","NE.3"):
    print(f"--- {veh} ---")
    rt=r.get(veh,[]); ct=c.get(veh,[])
    for i in range(max(len(rt),len(ct))):
        a=rt[i] if i<len(rt) else None
        b=ct[i] if i<len(ct) else None
        mark="" if a==b else "  <<< DIFF"
        print(f"ref={a} cand={b}{mark}")
PY
done
