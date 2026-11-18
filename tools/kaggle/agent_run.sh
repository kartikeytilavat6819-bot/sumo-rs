#!/bin/bash
# Gate diagnostic for oc57: full detail for the 10 failing tt_bugs rows only.
set -u
cd "$(dirname "$0")/../.." || exit 1
export PATH="$HOME/.cargo/bin:$PATH"
BIN="${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug"
SUMO="$BIN/sumo.exe"
[ -x "$SUMO" ] || SUMO="$BIN/sumo"
OUT=/tmp/oc57-scan
rm -rf "$OUT"; mkdir -p "$OUT" || exit 1
for c in \
  tt_bugs_13xxx_ticket13108 tt_bugs_15xxx_ticket15030 tt_bugs_15xxx_ticket15263 \
  tt_bugs_16xxx_ticket16635 tt_bugs_16xxx_ticket16641 tt_bugs_17xxx_ticket17480 \
  tt_bugs_17xxx_ticket17583 tt_bugs_2xxx_ticket2194 tt_bugs_4xxx_ticket4299 \
  tt_bugs_7xxx_ticket7149; do
  d=tools/oracle/cases/$c
  flags=$(cat "$d/sim.args" 2>/dev/null)
  timeout 60 "$SUMO" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $flags \
      --tripinfo-output "$OUT/ti.xml" --fcd-output "$OUT/fcd.xml" \
      --statistic-output "$OUT/stat.xml" --no-step-log true >"$OUT/$c.log" 2>&1
  rc=$?
  echo "===== $c rc=$rc ====="
  if [ $rc -ne 0 ]; then
    grep -A 14 -m1 -E 'panicked at|^Error:' "$OUT/$c.log" | head -16
    grep -m1 -E '^Error:' "$OUT/$c.log"
  else
    python tools/oracle/compare_sim.py --tripinfo "$d/ref.tripinfo.xml" "$OUT/ti.xml" | head -1
    [ -f "$d/ref.fcd.xml" ] && python tools/oracle/compare_sim.py --fcd "$d/ref.fcd.xml" "$OUT/fcd.xml" | head -1
    [ -f "$d/ref.stat.xml" ] && python tools/oracle/compare_stat.py "$d/ref.stat.xml" "$OUT/stat.xml" | head -1
  fi
done
