#!/bin/bash
# Gate diagnostic for oc59: run every active tt_tls case through our sumo and
# print a compact per-case PASS/FAIL with the first differing field.
set -u
cd "$(dirname "$0")/../.." || exit 1
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"; [ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc59-tls
rm -rf "$OUT"; mkdir -p "$OUT"
pass=0; fail=0; pending=0
for d in tools/oracle/cases/tt_tls_*/; do
  c=$(basename "$d")
  if [ -f "$d/PENDING" ]; then pending=$((pending+1)); continue; fi
  [ -f "$d/ref.tripinfo.xml" ] || continue
  flags=$(cat "$d/sim.args")
  timeout 120 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  if [ $rc -ne 0 ]; then
    printf '%-66s PANIC %s\n' "$c" "$(grep -m1 -E 'panicked at|Error:' "$OUT/$c.log" | cut -c1-70)"
    fail=$((fail+1)); continue
  fi
  detail=""
  if ! python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" >"$OUT/ti.cmp" 2>&1; then
    detail="ti: $(head -1 "$OUT/ti.cmp" | cut -c1-110)"
  elif [ -f "$d/ref.fcd.xml" ] && ! python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" >"$OUT/fcd.cmp" 2>&1; then
    detail="fcd: $(head -1 "$OUT/fcd.cmp" | cut -c1-110)"
  elif [ -f "$d/ref.stat.xml" ] && ! python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" >"$OUT/stat.cmp" 2>&1; then
    detail="stat: $(head -1 "$OUT/stat.cmp" | cut -c1-110)"
  fi
  if [ -z "$detail" ]; then
    printf '%-66s PASS\n' "$c"
    pass=$((pass+1))
  else
    printf '%-66s FAIL %s\n' "$c" "$detail"
    fail=$((fail+1))
  fi
done
echo "TOTAL pass=$pass fail=$fail pending=$pending"

# --- extra diagnostic: TLS switch timeline for NEMA_basic (root-cause of NEMA delays) ---
d=tools/oracle/cases/tt_tls_NEMA_basic
flags=$(cat "$d/sim.args")
"$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
    --tripinfo-output "$OUT/nema_ti.xml" --no-step-log true >"$OUT/nema_diag.log" 2>&1
echo "--- NEMA_basic our tls_switch.xml (first 24) ---"
grep '<tlsSwitch' "$d/tls_switch.xml" 2>/dev/null | head -24
echo "--- NEMA_basic our tripinfo (first 12) ---"
grep '<tripinfo' "$OUT/nema_ti.xml" 2>/dev/null | head -12
