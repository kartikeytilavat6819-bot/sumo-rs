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
P='ev=(WALK|MTNL|BADP|CWAF)'
echo "== ordered car12/person14 events at t=72000 ref"; grep 't=72000 ' "$OUT/ref.tr" | grep -E 'ev=(GLI|PBLK|BADP|CWAF|WALK|MTNL|GPED|PUT)' | head -30
echo "== ordered car12/person14 events at t=72000 cand"; grep 't=72000 ' "$OUT/cand.tr" | grep -E 'ev=(GLI|PBLK|BADP|CWAF|WALK|MTNL|GPED|PUT)' | head -30
echo "== person14 ref: $(grep -cE "$P" "$OUT/ref.tr") cand: $(grep -cE "$P" "$OUT/cand.tr")"
echo "== first diff of person14 lines"; diff <(grep -E "$P" "$OUT/ref.tr") <(grep -E "$P" "$OUT/cand.tr") | head -60
echo "== first 30 person14 ref"; grep -E "$P" "$OUT/ref.tr" | head -30
echo "== first 30 person14 cand"; grep -E "$P" "$OUT/cand.tr" | head -30
