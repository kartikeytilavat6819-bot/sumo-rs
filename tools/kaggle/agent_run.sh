#!/bin/bash
# TEMPORARY (sb03 docs measurement): prints every run_all verdict and the oracle summaries. Removed in the final commit.
cd "$(dirname "$0")/../.." || exit 1
bash tools/oracle/run_all.sh 2>&1 | grep -E 'PASS|FAIL|SKIP|PANIC'
echo "DEPART: $(bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1)"
OUT=/tmp/dua_sb03 timeout 600 bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
echo "DUAROUTER results.tsv:"; cat /tmp/dua_sb03/results.tsv
if [ -f tools/oracle/run_importers.sh ]; then
  echo "IMPORTERS:"; OUT=/tmp/imp_sb03 timeout 600 bash tools/oracle/run_importers.sh 2>&1 | tail -5
else
  echo "IMPORTERS: run_importers.sh absent"
fi
echo "OUTPUTS:"; OUT=/tmp/out_sb03 BIN="$CARGO_TARGET_DIR/debug/sumo" timeout 900 bash tools/oracle/run_outputs.sh 2>&1 | tail -5
echo "OUTPUTS results.tsv:"; tail -n +2 /tmp/out_sb03/results.tsv | cut -f2 | sort | uniq -c
