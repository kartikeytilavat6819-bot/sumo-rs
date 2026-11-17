#!/usr/bin/env bash
# TEMPORARY (fp02): car12 approaching-person trace on city_florence_pedestrians, official C++ vs ours. Remove before merge.
set -u
cd "$(dirname "$0")/../.." || exit 1
CASE=tools/oracle/cases/city_florence_pedestrians
OUT=/tmp/fp02; mkdir -p "$OUT"
BIN=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
CPP=$(bash tools/gcloud/vm_cpp_trace.sh fp02 tools/kaggle/fp02_cpp.patch) || { echo "CPP BUILD FAILED"; exit 1; }
ARGS="-n $CASE/ref_simple.net.xml -r $CASE/routes.rou --end 600 --no-step-log true"
timeout 900 "$CPP" $ARGS > "$OUT/ref.out" 2>&1; echo "ref rc=$?"
timeout 900 "$BIN" $ARGS > "$OUT/cand.out" 2>&1; echo "cand rc=$?"
grep -m3 'panicked' "$OUT/cand.out"
grep -h '^TRACE' "$OUT/ref.out" > "$OUT/ref.tr"; grep -h '^TRACE' "$OUT/cand.out" > "$OUT/cand.tr"
for e in REG UNREG GLI PBLK; do echo "== $e ref=$(grep -c "ev=$e " "$OUT/ref.tr") cand=$(grep -c "ev=$e " "$OUT/cand.tr")"; done
echo "== REG first 12 ref"; grep 'ev=REG' "$OUT/ref.tr" | head -12
echo "== REG first 12 cand"; grep 'ev=REG' "$OUT/cand.tr" | head -12
echo "== GLI/PBLK first 30 ref"; grep -E 'ev=(GLI|PBLK)' "$OUT/ref.tr" | head -30
echo "== GLI/PBLK first 30 cand"; grep -E 'ev=(GLI|PBLK)' "$OUT/cand.tr" | head -30
echo "== first diff of TRACE lines"; diff "$OUT/ref.tr" "$OUT/cand.tr" | head -40
