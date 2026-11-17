#!/bin/bash
# lo01 measurement (remove before merge): sublane_bikes car2 per-step SL2015 trace, ours vs instrumented C++
cd "$(dirname "$0")/../.." || exit 1
B=/tmp/target/debug
O=/tmp/lo01_out
mkdir -p "$O"
CPP=$(bash tools/gcloud/vm_cpp_trace.sh lo01 tools/kaggle/lc05_trace.patch 2>"$O/cppbuild.log")
echo "cpp bin: $CPP build-log lines: $(wc -l < "$O/cppbuild.log")"; head -5 "$O/cppbuild.log" | cut -c1-300
c=sublane_bikes
d=tools/oracle/cases/$c
export LCTRACE_ID=car2 LCTRACE_ID2=car1
RUST_BACKTRACE=1 "$B/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --fcd-output "$O/ours.fcd.xml" --no-step-log true >"$O/ours.out" 2>"$O/ours.trace"
echo "ours exit $? panic: $(grep -m1 -E 'panicked' "$O/ours.out" "$O/ours.trace" | cut -c1-160)"
"$CPP" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --fcd-output "$O/cpp.fcd.xml" --no-step-log true >"$O/cpp.out" 2>"$O/cpp.trace"
echo "cpp exit $? fcd-bytes=$(wc -c < "$O/cpp.fcd.xml" 2>/dev/null)"
echo "ref fcd==cpp: $(cmp -s "$d/ref.fcd.xml" "$O/cpp.fcd.xml" && echo yes || echo no) ours fcd==ref: $(cmp -s "$d/ref.fcd.xml" "$O/ours.fcd.xml" && echo yes || echo no)"
echo "car2 fcd diff lines: $(diff <(grep 'vehicle id="car2"' "$d/ref.fcd.xml") <(grep 'vehicle id="car2"' "$O/ours.fcd.xml") | wc -l)"
echo "LCT lines ours=$(grep -c '^LCT' "$O/ours.trace") cpp=$(grep -c '^LCT' "$O/cpp.trace")"
python - "$O/ours.trace" "$O/cpp.trace" <<'EOF'
import sys
def load(p): return [l.rstrip('\n') for l in open(p, errors='replace') if l.startswith('LCT')]
a = load(sys.argv[1]); b = load(sys.argv[2])
def toks(l): return l.replace(',', ' ').split()
seen = {}
for i, (x, y) in enumerate(zip(a, b)):
    xs = toks(x); ys = toks(y)
    vid = xs[2] if len(xs) > 2 else '?'
    if xs != ys and vid not in seen:
        j = next((k for k, (p, q) in enumerate(zip(xs, ys)) if p != q), min(len(xs), len(ys)))
        seen[vid] = i + 1
        print('first diff', vid, 'line', i + 1, 'token', j, 'ours', xs[j] if j < len(xs) else '-', 'cpp', ys[j] if j < len(ys) else '-')
        print('  ours:', x[:260]); print('  cpp :', y[:260])
print('diverged ids', seen)
EOF
