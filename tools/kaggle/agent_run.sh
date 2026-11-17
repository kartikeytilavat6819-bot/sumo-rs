#!/bin/bash
# TEMPORARY sc08 measurement: MSLink::opened decisions in checkChange (checkOpened), ours vs instrumented C++.
set -u
ROOT=$(pwd)
OURS=${CARGO_TARGET_DIR:-/tmp/target}/debug/sumo
CPP=$(bash tools/gcloud/vm_cpp_trace.sh sc08 tools/oracle/trace/sc08_opened.patch) || { echo "sc08: C++ trace build failed"; exit 1; }
OUT=/tmp/sc08
mkdir -p "$OUT"
for c in city_plano city_bengaluru_blr; do
  cd "$ROOT/tools/oracle/cases/$c" || exit 1
  SC08_TRACE=1 "$OURS" -n ref_simple.net.xml -r routes.rou $(cat sim.args) --no-step-log true >"$OUT/$c.ours.out" 2>"$OUT/$c.ours.err"
  SC08_TRACE=1 "$CPP" -n ref_simple.net.xml -r routes.rou $(cat sim.args) --no-step-log true >"$OUT/$c.cpp.out" 2>"$OUT/$c.cpp.err"
  echo "== $c ours=$(grep -c '^OPENED' "$OUT/$c.ours.err") cpp=$(grep -c '^OPENED' "$OUT/$c.cpp.err")"
  diff <(grep '^OPENED' "$OUT/$c.ours.err") <(grep '^OPENED' "$OUT/$c.cpp.err") | head -12
done
