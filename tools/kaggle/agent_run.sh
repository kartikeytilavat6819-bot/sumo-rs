#!/usr/bin/env bash
# oc104 probe: tt_devices_ssm_bugs_ticket5231 with the quoting the original options.sumo has
# ('TTC DRAC', '5 3'); the imported sim.args lost it. Official vs ours, first differences.
OUT=/tmp/oc104; mkdir -p "$OUT"
ROOT=$(pwd)
OFF="${SUMO_PRISTINE_BIN:?}/sumo"
OURS=$(readlink -f "${CARGO_TARGET_DIR:-target}/debug/sumo")
c=tt_devices_ssm_bugs_ticket5231
d="$ROOT/tools/oracle/cases/$c"
for side in off ours; do
  bin=$OFF; [ $side = ours ] && bin=$OURS
  rdir="$OUT/$c.$side"; rm -rf "$rdir"; mkdir -p "$rdir"; cp -r "$d/." "$rdir/"
  (cd "$rdir" && timeout 120 "$bin" -n ref_simple.net.xml -r routes.rou --no-step-log true \
      --write-license --default.speeddev 0 --default.departspeed 0 --default.departlane first \
      --device.ssm.probability 1 --device.ssm.measures 'TTC DRAC' --device.ssm.thresholds '5 3' \
      --device.ssm.range 200 --device.ssm.file ssm.xml >out.txt 2>err.txt; echo $? > rc.txt)
  echo "== $side: exit $(cat "$rdir/rc.txt") stderr lines: $(wc -l < "$rdir/err.txt")"
  head -n 4 "$rdir/err.txt" | cut -c1-200 | sed 's/^/     /'
done
echo "-- stderr diff (off vs ours):"; diff "$OUT/$c.off/err.txt" "$OUT/$c.ours/err.txt" | head -n 10 | cut -c1-200
echo "-- stdout diff (off vs ours, durations masked):"
diff <(sed -E 's/\([0-9]+ms\)|[0-9.]+ms//g; s/ref_simple/net/g' "$OUT/$c.off/out.txt") <(sed -E 's/\([0-9]+ms\)|[0-9.]+ms//g; s/ref_simple/net/g' "$OUT/$c.ours/out.txt") | head -n 10 | cut -c1-200
echo "-- ssm.xml diff (off vs ours):"
if [ -f "$OUT/$c.off/ssm.xml" ] && [ -f "$OUT/$c.ours/ssm.xml" ]; then
  diff "$OUT/$c.off/ssm.xml" "$OUT/$c.ours/ssm.xml" | head -n 12 | cut -c1-200
else
  echo "ssm.xml: off=$([ -f "$OUT/$c.off/ssm.xml" ] && echo yes || echo no) ours=$([ -f "$OUT/$c.ours/ssm.xml" ] && echo yes || echo no)"
fi
echo "-- ref.errors committed: $(wc -c < "$d/ref.errors") bytes"
