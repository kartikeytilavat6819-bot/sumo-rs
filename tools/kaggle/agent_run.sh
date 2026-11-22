#!/bin/bash
# oc97: sharper probe -- per-step posLat, C++ vs ours, for the three moped indirect-left cases.
# Runs both binaries with --fcd-output.attributes posLat and prints the named vehicle's
# lateral position around the divergence, plus our env-gated decision trace.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}/debug"
OUT="/tmp/oc97-fcd-$$"
mkdir -p "$OUT"
trace() {
  c="$1"; vid="$2"; scope="$3"
  dir="tools/oracle/cases/$c/"
  echo "=== $c (vehicle $vid) ==="
  EXTRA=""
  [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
  for side in cpp ours; do
    if [ "$side" = cpp ]; then B="${SUMO_PRISTINE_BIN:?no SUMO_PRISTINE_BIN}/sumo"; else B="$BIN/sumo"; fi
    if [ "$side" = cpp ]; then
      timeout 240 "$B" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --fcd-output "$OUT/$c.$side.xml" --fcd-output.attributes posLat,x,y,speed,pos,lane,accel,accelLat \
        --fcd-output.acceleration true --no-step-log true $EXTRA > "$OUT/$c.$side.log" 2>&1
    else
      OC97_TRACE="$scope" timeout 240 "$B" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
        --fcd-output "$OUT/$c.$side.xml" --fcd-output.attributes posLat,x,y,speed,pos,lane,accel,accelLat \
        --fcd-output.acceleration true --no-step-log true $EXTRA > "$OUT/$c.$side.log" 2>&1
    fi
    echo "  run $side rc=$?"
  done
  echo "  --- posLat cpp | ours ---"
  python - "$OUT/$c.cpp.xml" "$OUT/$c.ours.xml" "$vid" <<'PY'
import re, sys
cpp, ours, vid = sys.argv[1:4]
needle = 'id="%s"' % vid

def extract(path):
    out, t = {}, None
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.search(r'<timestep\s+time="([^"]+)"', line)
            if m:
                t = round(float(m.group(1)), 2)
            if '<vehicle' in line and needle in line:
                p = re.search(r'posLat="([^"]+)"', line)
                al = re.search(r'accelLat="([^"]+)"', line)
                v = re.search(r'speed="([^"]+)"', line)
                if p and t is not None:
                    out[t] = (float(p.group(1)), float(al.group(1)) if al else 0.0, float(v.group(1)) if v else 0.0)
    return out

a, b = extract(cpp), extract(ours)
for t in sorted(set(a) | set(b)):
    if not (137.5 <= t <= 145.5 or 411.5 <= t <= 417.5):
        continue
    x, y = a.get(t), b.get(t)
    mark = "" if x == y else "   <-- DIFF"
    fmt = lambda q: ("%.4f/%.3f/%.2f" % q) if q is not None else "-"
    print("  t=%6.2f  cpp[posLat/accLat/v]=%16s  ours=%16s%s" % (t, fmt(x), fmt(y), mark))
PY
  echo "  --- our decision trace (csl/lat/align/foeTrace only) ---"
  grep -E '^OC97 .*(csl|latDone| lat |alignSet|foeTrace)' "$OUT/$c.ours.log" | head -60 || true
}

trace tt_sublane_model_junction_indirect_left_moped                       flow_0.15 sub
trace tt_sublane_model_junction_turnAlignment_moped_indirect_left         flow_0.16 sub
trace tt_junction_model_indirect_left_turn_internal_junction_controlled_moped flow_0.52 jun
