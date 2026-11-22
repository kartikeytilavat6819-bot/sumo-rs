#!/bin/bash
# oc97: sharper probe -- per-step posLat, C++ vs ours, for the three moped indirect-left cases.
# Runs both binaries with --fcd-output.attributes posLat and prints the named vehicle's
# lateral position around the divergence, plus our env-gated decision trace.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}/debug"
OUT="/tmp/oc97-fcd-$$"
mkdir -p "$OUT"
export OC97_TRACE=1

trace() {
  c="$1"; vid="$2"
  dir="tools/oracle/cases/$c/"
  echo "=== $c (vehicle $vid) ==="
  EXTRA=""
  [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
  for side in cpp ours; do
    if [ "$side" = cpp ]; then B="${SUMO_PRISTINE_BIN:?no SUMO_PRISTINE_BIN}/sumo"; else B="$BIN/sumo"; fi
    timeout 240 "$B" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --fcd-output "$OUT/$c.$side.xml" --fcd-output.attributes posLat \
      --no-step-log true $EXTRA > "$OUT/$c.$side.log" 2>&1
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
                if p and t is not None:
                    out[t] = float(p.group(1))
    return out

a, b = extract(cpp), extract(ours)
for t in sorted(set(a) | set(b)):
    if not (137.5 <= t <= 145.5 or 411.5 <= t <= 417.5):
        continue
    x, y = a.get(t), b.get(t)
    mark = "" if x == y else "   <-- DIFF"
    print("  t=%6.2f  cpp=%9s  ours=%9s%s" % (
        t, "%.4f" % x if x is not None else "-", "%.4f" % y if y is not None else "-", mark))
PY
  echo "  --- our decision trace ---"
  grep '^OC97 ' "$OUT/$c.ours.log" | head -50 || true
}

trace tt_sublane_model_junction_indirect_left_moped                       flow_0.15
trace tt_sublane_model_junction_turnAlignment_moped_indirect_left         flow_0.16
trace tt_junction_model_indirect_left_turn_internal_junction_controlled_moped flow_0.52
