#!/bin/bash
# Trace the first FCD divergence for the three oc97 moped indirect-left rows.
# Runs our Rust sumo with --fcd-output (same invocation as tools/oracle/run_all.sh)
# and compares the named vehicle's FCD lines against the case's ref.fcd.xml.
# Prints, per case, the first differing timestep and both full vehicle lines.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/tmp/sumo-rs-target}/debug"
OUT="/tmp/oc97-fcd-$$"
mkdir -p "$OUT"
export OC97_TRACE=1

# Emit, for one vehicle id, a map timestep -> stripped <vehicle> line, then the
# first timestep where the two maps differ (including presence/absence).
first_diff() {
  python - "$1" "$2" "$3" <<'PY'
import re, sys
ref_path, our_path, vid = sys.argv[1], sys.argv[2], sys.argv[3]
needle = 'id="%s"' % vid

def extract(path):
    out, t = {}, None
    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = re.search(r'<timestep\s+time="([^"]+)"', line)
            if m:
                t = m.group(1)
            if '<vehicle' in line and needle in line:
                out[t] = " ".join(line.split())
    return out

ref, our = extract(ref_path), extract(our_path)
for t in sorted(set(ref) | set(our), key=float):
    a, b = ref.get(t), our.get(t)
    if a != b:
        print("  FIRST DIFF t=%s" % t)
        print("    REF : %s" % (a if a is not None else "<absent>"))
        print("    OURS: %s" % (b if b is not None else "<absent>"))
        break
else:
    print("  NO DIFF in the named vehicle's FCD lines")
PY
}

trace() {
  c="$1"; vid="$2"
  dir="tools/oracle/cases/$c/"
  echo "=== $c (vehicle $vid) ==="
  if [ ! -f "$dir/ref.fcd.xml" ]; then echo "  no ref.fcd.xml"; return; fi
  if [ ! -x "$BIN/sumo" ]; then echo "  no $BIN/sumo"; return; fi
  EXTRA=""
  [ -f "$dir/sim.args" ] && EXTRA=$(cat "$dir/sim.args")
  timeout 240 "$BIN/sumo" -n "$dir/ref_simple.net.xml" -r "$dir/routes.rou" \
      --fcd-output "$OUT/$c.fcd.xml" --no-step-log true $EXTRA \
      >"$OUT/$c.sumo.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then
    echo "  SUMO FAILED rc=$rc: $(grep -m1 -E 'panicked at|Error:' "$OUT/$c.sumo.log" | cut -c1-120)"
    return
  fi
  if [ ! -s "$OUT/$c.fcd.xml" ]; then echo "  EMPTY/MISSING our fcd"; return; fi
  echo "  --- trace ---"
  grep '^OC97 ' "$OUT/$c.sumo.log" || true
  echo "  --- first diff ---"
  first_diff "$dir/ref.fcd.xml" "$OUT/$c.fcd.xml" "$vid"
}

trace tt_sublane_model_junction_indirect_left_moped                       flow_0.15
trace tt_sublane_model_junction_turnAlignment_moped_indirect_left         flow_0.16
trace tt_junction_model_indirect_left_turn_internal_junction_controlled_moped flow_0.52
