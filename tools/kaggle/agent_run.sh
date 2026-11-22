#!/bin/bash
# oc102: trace the first FCD difference for tt_devices_bluelight_3lanes_left.
# Runs on the Linux gate from the repo root. Writes only under /tmp/oc102.
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc102
mkdir -p "$OUT"

CASE=tools/oracle/cases/tt_devices_bluelight_3lanes_left
REF="$CASE/ref.fcd.xml"
CAND="$OUT/ours.fcd.xml"

BIN="${CARGO_TARGET_DIR:?CARGO_TARGET_DIR must be set}/debug/sumo"
if [ ! -x "$BIN" ] && [ -x "$BIN.exe" ]; then BIN="$BIN.exe"; fi

EXTRA=$(cat "$CASE/sim.args")
OC102_DEBUG=1 timeout 600 "$BIN" -n "$CASE/ref_simple.net.xml" -r "$CASE/routes.rou" \
    --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$CAND" \
    --statistic-output "$OUT/ours.stat.xml" --no-step-log true $EXTRA \
    >"$OUT/ours.log" 2>&1
echo "sumo rc=$? ; OC102 trace (t=12..27, raw steps 12000..27000):"
grep -E 'OC102 t=(1[2-9]|2[0-7])000 |OC102  (reset|influence|resetVehicle)' "$OUT/ours.log" 2>/dev/null || true

PY=python3; command -v python3 >/dev/null || PY=python
"$PY" - "$REF" "$CAND" <<'PYEOF'
import re, sys

def parse(p):
    steps, cur = {}, None
    with open(p, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.search(r'<timestep time="([^"]+)"', line)
            if m:
                cur = round(float(m.group(1)), 2)
                steps.setdefault(cur, {})
                continue
            m = re.search(r'<vehicle id="([^"]+)"', line)
            if m and cur is not None:
                steps[cur][m.group(1)] = line.strip()
    return steps

def edge(line):
    if not line: return ""
    m = re.search(r'lane="([^"]+)"', line)
    return m.group(1).rsplit("_", 1)[0] if m else ""

ref, cand = parse(sys.argv[1]), parse(sys.argv[2])
times = sorted(set(ref) | set(cand))

def differs(a, b, t):
    ra, cb = ref.get(t, {}), cand.get(t, {})
    for vid in sorted(set(ra) | set(cb)):
        if ra.get(vid) != cb.get(vid):
            return vid
    return None

# first difference involving rescue or anything on the edge rescue is on
first_rel = None
for t in times:
    ra, cb = ref.get(t, {}), cand.get(t, {})
    edges = {edge(ra.get("rescue")), edge(cb.get("rescue"))} - {""}
    for vid in sorted(set(ra) | set(cb)):
        if ra.get(vid) == cb.get(vid): continue
        if vid == "rescue" or edge(ra.get(vid)) in edges or edge(cb.get(vid)) in edges:
            first_rel = t; break
    if first_rel is not None: break

first_any = next((t for t in times if differs(ref, cand, t) is not None), None)
print("FIRST_DIFF_ANY=%s" % first_any)
print("FIRST_DIFF_RESCUE=%s" % first_rel)
t = first_rel if first_rel is not None else first_any
if t is None:
    print("NO DIFFERENCE FOUND"); sys.exit(0)

ra, cb = ref.get(t, {}), cand.get(t, {})
edges = {edge(ra.get("rescue")), edge(cb.get("rescue"))} - {""}
print("--- timestep %.2f ---" % t)
print("rescue edges: %s" % sorted(edges))
for vid in sorted(set(ra) | set(cb)):
    same = ra.get(vid) == cb.get(vid)
    ctx = vid == "rescue" or edge(ra.get(vid)) in edges or edge(cb.get(vid)) in edges
    if same and not ctx: continue
    mark = "=" if same else "DIFF"
    print("%s %s:" % (vid, mark))
    print("  REF  %s" % ra.get(vid, "<absent>"))
    print("  CAND %s" % cb.get(vid, "<absent>"))
PYEOF

# First FCD difference for the other bluelight target cases (same root cause?).
for c in tt_devices_bluelight_3lanes_right tt_devices_bluelight_3lanes_straight_blocked tt_devices_bluelight_3lanes_straight_blocked_default tt_devices_bluelight_turnlane_blocked tt_devices_bluelight_bugs_ticket9310 tt_devices_bluelight_opposite_multiple2; do
  D=tools/oracle/cases/$c
  [ -f "$D/ref.fcd.xml" ] || continue
  FLAGS=$(cat "$D/sim.args" 2>/dev/null)
  timeout 300 "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" \
      --fcd-output "$OUT/$c.fcd.xml" --no-step-log true $FLAGS >"$OUT/$c.log" 2>&1
  echo "====FCDDIFF $c rc=$?"
  "$PY" - "$D/ref.fcd.xml" "$OUT/$c.fcd.xml" <<'PYEOF2'
import re,sys
def parse(p):
    d={}; t=None
    with open(p,encoding="utf-8",errors="replace") as f:
        for l in f:
            m=re.search(r'<timestep time="([^"]+)"',l)
            if m: t=round(float(m.group(1)),2); d.setdefault(t,{}); continue
            m=re.search(r'<vehicle id="([^"]+)"',l)
            if m and t is not None: d[t][m.group(1)]=l.strip()
    return d
a,b=parse(sys.argv[1]),parse(sys.argv[2])
for t in sorted(set(a)|set(b)):
    ra,rb=a.get(t,{}),b.get(t,{})
    for v in sorted(set(ra)|set(rb)):
        if ra.get(v)!=rb.get(v):
            print("FIRST t=%s veh=%s"%(t,v))
            print(" REF ",ra.get(v,"<absent>"))
            print(" CAND",rb.get(v,"<absent>"))
            sys.exit(0)
print("NO DIFF")
PYEOF2
done

# oc105 measurement (merged from wave2): compare our sumo with the official binary.
OUT=/tmp/oc105
mkdir -p "$OUT"
OUR="$CARGO_TARGET_DIR/debug/sumo"
OFF="$SUMO_PRISTINE_BIN/sumo"
for c in tt_devices_ssm_bugs_ticket13918 tt_devices_rerouting_reroute_congestions_exit_times; do
  D="tools/oracle/cases/$c"
  FLAGS=$(cat "$D/sim.args")
  echo "==== $c"
  echo "sim.args: $FLAGS"
  for side in our official; do
    if [ "$side" = our ]; then BIN="$OUR"; else BIN="$OFF"; fi
    "$BIN" -n "$D/ref_simple.net.xml" -r "$D/routes.rou" --no-step-log true $FLAGS \
      >"$OUT/$c.$side.out" 2>"$OUT/$c.$side.err"
    echo "$side exit=$?"
    echo "$side stderr:"; head -20 "$OUT/$c.$side.err"
    echo "$side stdout:"; head -12 "$OUT/$c.$side.out"
  done
done
