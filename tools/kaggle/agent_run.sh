#!/bin/bash
# Gate diagnostic for oc60: run one or more imported oracle rows with our binary
# and print the first difference against the committed official reference.
# Target cases: one per line in tools/kaggle/agent_cases.txt, optionally "case|vehicle"
# to also dump that vehicle's full tripinfo attributes from ref and candidate.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="/c/Users/karti/.cargo/bin:$PATH"
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=tools/oracle/.oc60diag
rm -rf "$OUT"; mkdir -p "$OUT"
total=$(find tools/oracle/cases -maxdepth 1 -type d -name 'tt_*' | wc -l)
echo "tt cases=$total"

while read -r entry; do
  entry=$(echo "$entry" | tr -d '\r')
  [ -n "$entry" ] || continue
  c=${entry%%|*}
  veh=""
  [ "$entry" != "$c" ] && veh=${entry#*|}
  d=tools/oracle/cases/$c
  [ -f "$d/ref.tripinfo.xml" ] || { echo "$c: NO CASE/REF"; continue; }
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout "${SIM_TIMEOUT:-120}" "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/$c.ti.xml" --fcd-output "$OUT/$c.fcd.xml" \
      --statistic-output "$OUT/$c.stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  printf '%-62s rc=%s ' "$c" "$rc"
  if [ "$rc" -ne 0 ]; then
    grep -m1 -E 'panicked at|Error:|Fatal' "$OUT/$c.log" | cut -c1-160
    grep -A12 'panicked at' "$OUT/$c.log" | grep -E 'ms_|geo_conv|ptr.rs|getFreeLane|check_insertions' | head -4 | sed 's/^/    /'
    continue
  fi
  d1=$(python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" 2>&1 | head -1 | cut -c1-220)
  echo "tripinfo:${d1:-MATCH}"
  if [ -f "$d/ref.fcd.xml" ]; then
    d2=$(python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/$c.fcd.xml" 2>&1 | head -1 | cut -c1-160)
    echo "  fcd:${d2:-MATCH}"
  fi
  if [ -n "$veh" ]; then
    python - "$veh" "$d/ref.tripinfo.xml" "$OUT/$c.ti.xml" <<'PY'
import sys, xml.etree.ElementTree as ET
vid, ref, cand = sys.argv[1], sys.argv[2], sys.argv[3]
def attrs(path):
    out = {}
    for e in ET.parse(path).getroot().iter('tripinfo'):
        if e.get('id') == vid:
            out[vid] = dict(e.attrib)
    return out.get(vid, {})
a, b = attrs(ref), attrs(cand)
print(f"  [{vid}] ref:   " + " ".join(f"{k}={v}" for k,v in a.items()))
print(f"  [{vid}] cand:  " + " ".join(f"{k}={v}" for k,v in b.items()))
print(f"  [{vid}] DIFF:  " + " ".join(f"{k}: {a.get(k)}!={b.get(k)}" for k in sorted(set(a)|set(b)) if a.get(k)!=b.get(k)))
PY
  fi
done < tools/kaggle/agent_cases.txt
