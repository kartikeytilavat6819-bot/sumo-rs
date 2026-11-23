#!/bin/bash
# oc108: measure polyconvert OSM import + duarouter meta config rows (official vs ours).
set -u
cd "$(dirname "$0")/../.." || exit 1
OUT=/tmp/oc108tools
rm -rf "$OUT"; mkdir -p "$OUT"
norm() { grep -v 'generated on' "$1" | sed -E 's#/(ref|cand)/#/SIDE/#g'; }
measure() {
  local tool=$1 c=$2
  local cdir=tools/oracle/cases/$c sub=$OUT/$c
  local args rargs cargs rr cr
  args=$(tr -d '\r' < "$cdir/args")
  rargs=${args//@DIR@/$sub/ref}; cargs=${args//@DIR@/$sub/cand}
  mkdir -p "$sub/ref" "$sub/cand"
  "$SUMO_PRISTINE_BIN/$tool" $rargs >"$sub/ref.log" 2>&1; rr=$?
  "$CARGO_TARGET_DIR/debug/$tool" $cargs >"$sub/cand.log" 2>&1; cr=$?
  echo "==== $c ($tool) ref_rc=$rr cand_rc=$cr"
  echo " ref files: $(ls "$sub/ref" | tr '\n' ' ')"
  echo " cand files: $(ls "$sub/cand" | tr '\n' ' ')"
  for f in $(ls "$sub/ref"); do
    if [ -f "$sub/cand/$f" ]; then
      diff -u <(norm "$sub/ref/$f") <(norm "$sub/cand/$f") | head -30
    fi
  done
  grep -iE 'panic|error|warning' "$sub/cand.log" | head -8
}
for c in polyconvert_itt_polyconvert_import_OSM_buildings_from_relation \
         polyconvert_itt_polyconvert_import_OSM_inner_shape \
         polyconvert_itt_polyconvert_import_OSM_water_incomplete; do
  measure polyconvert "$c"
done
measure duarouter duarouter_itt_duarouter_meta_write_config

# Reproduce the exact duarouter-oracle comparison (it adds -n and the oracle normalizer).
DC=duarouter_itt_duarouter_meta_write_config
bash tools/oracle/run_duarouter.sh "$DC" || true
echo "==== raw oracle config diff"
cat "/tmp/duarouter_oracle/$DC/config.cfg.diff" 2>/dev/null || true
echo "==== raw ref config lines"
sed -n '1,16p' "/tmp/duarouter_oracle/$DC/ref/config.cfg" 2>/dev/null || true
echo "==== raw cand config lines"
sed -n '1,16p' "/tmp/duarouter_oracle/$DC/cand/config.cfg" 2>/dev/null || true
