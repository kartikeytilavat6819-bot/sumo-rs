#!/bin/bash
# tx02 trace: taxi0 parking re-insertion, ours vs instrumented C++ (temporary, removed before merge)
OUT=/tmp/tx02
mkdir -p "$OUT"
BIN="$CARGO_TARGET_DIR/debug/sumo"
[ -x "$BIN" ] || BIN="$BIN.exe"
CPP=$(bash tools/gcloud/vm_cpp_trace.sh tx02 tools/kaggle/tx02_cpp_trace.patch 2>"$OUT/build.log" | tail -n 1)
echo "== cpp binary: $CPP"
cd tools/oracle/cases/taxi || exit 1
ARGS=$(cat sim.args)
TX02_TRACE=1 "$BIN" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$OUT/ours.ti.xml" --no-step-log true $ARGS >"$OUT/ours.log" 2>&1
echo "== ours exit=$?"
"$CPP" -n ref_simple.net.xml -r routes.rou --tripinfo-output "$OUT/cpp.ti.xml" --no-step-log true $ARGS >"$OUT/cpp.log" 2>&1
echo "== cpp exit=$?"
echo "-- TX02 line counts (ours cpp)"
grep -c 'TX02' "$OUT/ours.log"
grep -c 'TX02' "$OUT/cpp.log"
echo "-- first divergence"
diff <(grep 'TX02' "$OUT/ours.log") <(grep 'TX02' "$OUT/cpp.log") | head -n 40
echo "-- ours park"
grep 'TX02 park' "$OUT/ours.log" | head -n 20
echo "-- cpp park"
grep 'TX02 park' "$OUT/cpp.log" | head -n 20
echo "-- tripinfo taxi0"
grep 'tripinfo id="taxi0"' "$OUT/ours.ti.xml" | cut -c1-200
grep 'tripinfo id="taxi0"' "$OUT/cpp.ti.xml" | cut -c1-200
