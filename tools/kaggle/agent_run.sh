#!/bin/bash
# du04 scratch runner (removed in the final commit): duarouter oracle on every case, verdict table only
cd "$(dirname "$0")/../.." || exit 1
B=${CARGO_TARGET_DIR:-/tmp/target}/debug/duarouter
echo "bin: $(ls -l --time-style=+%T "$B" 2>&1)  now: $(date +%T)"
echo "stale-text hits: $(strings "$B" 2>/dev/null | grep -c 'is not wired in duarouter yet')  fresh-text hits: $(strings "$B" 2>/dev/null | grep -c 'emission effort')"
L=$(ls -t /tmp/build*.log 2>/dev/null | head -1); echo "build log $L:"; tail -4 "$L"
bash tools/oracle/run_duarouter.sh 2>&1 | tail -40
awk -F'\t' 'NR>1{n++; if($2=="PASS")p++} END{print "duarouter PASS=" p "/" n}' "${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}/results.tsv"
