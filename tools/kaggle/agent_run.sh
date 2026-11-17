#!/bin/bash
# du04 scratch runner (removed in the final commit): duarouter oracle on every case, verdict table only
cd "$(dirname "$0")/../.." || exit 1
bash tools/oracle/run_duarouter.sh 2>&1 | tail -40
awk -F'\t' 'NR>1{n++; if($2=="PASS")p++} END{print "duarouter PASS=" p "/" n}' "${OUT:-${TMPDIR:-/tmp}/duarouter_oracle}/results.tsv"
