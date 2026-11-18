#!/bin/bash
# oc11 diagnostic: run our sumo and the pristine C++ sumo on meso_calibrator_edgedata and diff outputs.
set -u
cd "$(dirname "$0")/../.." || exit 1
C=tools/oracle/cases/meso_calibrator_edgedata
OURS=${OURS:-${CARGO_TARGET_DIR:-/c/Users/karti/sumo-rs-target}/debug/sumo.exe}
PRIS=${SUMO_PRISTINE_BIN:-$HOME/sumo-pristine/bin}/sumo
export SUMO_HOME=${SUMO_HOME:-$HOME/sumo-pristine/lib/python3.12/site-packages/sumo}
OUT=/tmp/oc11_meso
rm -rf "$OUT"; mkdir -p "$OUT"
EXTRA=$(cat "$C/sim.args")
for side in ref cand; do
  if [ "$side" = cand ]; then BIN=$OURS; else BIN=$PRIS; fi
  rm -f "$C/ed0.xml" "$C/ld0.xml"
  "$BIN" -n "$C/ref_simple.net.xml" -r "$C/routes.rou" \
      --tripinfo-output "$OUT/$side.ti.xml" --fcd-output "$OUT/$side.fcd.xml" \
      --statistic-output "$OUT/$side.stat.xml" --no-step-log true $EXTRA \
      > "$OUT/$side.log" 2>&1
  echo "== $side exit=$? bin=$BIN"
  cp "$C/ed0.xml" "$OUT/$side.ed0.xml" 2>/dev/null
  cp "$C/ld0.xml" "$OUT/$side.ld0.xml" 2>/dev/null
done
rm -f "$C/ed0.xml" "$C/ld0.xml"
for f in ti fcd stat ed0 ld0; do
  echo "==== diff $f ===="
  diff "$OUT/ref.$f.xml" "$OUT/cand.$f.xml" | head -40
done
