#!/bin/bash
# tx01 trace: taxi panic backtrace + first tripinfo diff (temporary, removed before merge)
cd tools/oracle/cases/taxi || exit 1
BIN="$CARGO_TARGET_DIR/debug"
OUT=/tmp/tx01
mkdir -p "$OUT"
for tag in ref ref_closest; do
  argsf=sim.args; [ "$tag" = ref_closest ] && argsf=sim_closest.args
  RUST_BACKTRACE=1 "$BIN/sumo.exe" -n ref_simple.net.xml -r routes.rou \
    --tripinfo-output "$OUT/$tag.ti.xml" --fcd-output "$OUT/$tag.fcd.xml" \
    --statistic-output "$OUT/$tag.stat.xml" --no-step-log true $(cat "$argsf") >"$OUT/$tag.log" 2>&1
  echo "== $tag exit=$?"
  grep -m3 -E 'panicked at|Error:' "$OUT/$tag.log"
  grep -E 'sumo_core|crates/' "$OUT/$tag.log" | head -30
  echo "-- tripinfo diff ($tag)"
  diff "$tag.tripinfo.xml" "$OUT/$tag.ti.xml" | head -30
done
