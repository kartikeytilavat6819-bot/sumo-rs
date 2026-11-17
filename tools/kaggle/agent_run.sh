#!/bin/bash
# lc05 measurement (remove before merge): SL2015 per-step trace, ours vs instrumented C++ (LCTRACE_ID)
cd "$(dirname "$0")/../.." || exit 1
B=/tmp/target/debug
O=/tmp/lc05_out
mkdir -p "$O"
CPP=$(bash tools/gcloud/vm_cpp_trace.sh lc05 tools/kaggle/lc05_trace.patch 2>"$O/cppbuild.log")
echo "cpp bin: $CPP build-log lines: $(wc -l < "$O/cppbuild.log")"; head -5 "$O/cppbuild.log" | cut -c1-300
for spec in "sl2015_lat04 m5" "sl2015_lat08 c2"; do
  set -- $spec
  c=$1; id=$2
  d=tools/oracle/cases/$c
  LCTRACE_ID=$id "$B/sumo" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --fcd-output "$O/$c.ours.fcd.xml" --no-step-log true >"$O/$c.ours.out" 2>"$O/$c.ours.trace"
  echo "$c ours exit $? panic: $(grep -m1 -E 'panicked' "$O/$c.ours.out" | cut -c1-160)"
  LCTRACE_ID=$id "$CPP" -n "$d/ref_simple.net.xml" -r "$d/routes.rou" $(cat "$d/sim.args") --fcd-output "$O/$c.cpp.fcd.xml" --no-step-log true >"$O/$c.cpp.out" 2>"$O/$c.cpp.trace"
  echo "$c cpp exit $?"
  echo "$c cpp ver: $("$CPP" --version 2>&1 | head -1 | cut -c1-100) fcd-bytes=$(wc -c < "$O/$c.cpp.fcd.xml" 2>/dev/null) cppstderr=$(head -c 300 "$O/$c.cpp.trace" | tr '\n' ' ')"
  echo "$c cpp diag: strings=$(strings "$CPP" | grep -c 'LCT-CONT') trace-bytes=$(wc -c < "$O/$c.cpp.trace") fcd-diff-lines=$(diff "$d/ref.fcd.xml" "$O/$c.cpp.fcd.xml" | wc -l) out=$(head -c 300 "$O/$c.cpp.out" | tr '\n' ' ')"
  cmp -s "$d/ref.fcd.xml" "$O/$c.cpp.fcd.xml" && echo "$c cpp fcd == ref" || echo "$c cpp fcd != ref"
  cmp -s "$d/ref.fcd.xml" "$O/$c.ours.fcd.xml" && echo "$c ours fcd == ref" || echo "$c ours fcd != ref"
  echo "$c LCT lines ours=$(grep -c '^LCT' "$O/$c.ours.trace") cpp=$(grep -c '^LCT' "$O/$c.cpp.trace")"
  diff <(grep '^LCT' "$O/$c.ours.trace") <(grep '^LCT' "$O/$c.cpp.trace") | head -4 | cut -c1-200
  python - "$O/$c.ours.trace" "$O/$c.cpp.trace" <<'EOF'
import sys
def load(p): return [l.rstrip('\n') for l in open(p) if l.startswith('LCT')]
a = load(sys.argv[1]); b = load(sys.argv[2])
def toks(l): return l.replace(',', ' ').split()
for i, (x, y) in enumerate(zip(a, b)):
    xs = toks(x); ys = toks(y)
    if xs != ys:
        j = next((k for k, (p, q) in enumerate(zip(xs, ys)) if p != q), min(len(xs), len(ys)))
        print('first real diff line', i + 1, 'token', j, 'ours', xs[j] if j < len(xs) else '-', 'cpp', ys[j] if j < len(ys) else '-', 'ntok', len(xs), len(ys))
        print('ours:', x[:300]); print('cpp :', y[:300])
        break
else:
    print('no real diff in common prefix', len(a), len(b))
EOF
done
