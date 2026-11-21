#!/bin/bash
# oc102: trace the first FCD difference for tt_devices_bluelight_3lanes_left.
# Runs on the Linux gate from the repo root. Writes only under /tmp/oc102.
set -u
OUT=/tmp/oc102
mkdir -p "$OUT"

CASE=tools/oracle/cases/tt_devices_bluelight_3lanes_left
REF="$CASE/ref.fcd.xml"
CAND="$OUT/ours.fcd.xml"

BIN="${CARGO_TARGET_DIR:?CARGO_TARGET_DIR must be set}/debug/sumo"
if [ ! -x "$BIN" ] && [ -x "$BIN.exe" ]; then BIN="$BIN.exe"; fi

EXTRA=$(cat "$CASE/sim.args")
timeout 600 "$BIN" -n "$CASE/ref_simple.net.xml" -r "$CASE/routes.rou" \
    --tripinfo-output "$OUT/ours.ti.xml" --fcd-output "$CAND" \
    --statistic-output "$OUT/ours.stat.xml" --no-step-log true $EXTRA \
    >"$OUT/ours.log" 2>&1
echo "sumo rc=$? ; log tail:"
tail -5 "$OUT/ours.log" 2>/dev/null || true

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
