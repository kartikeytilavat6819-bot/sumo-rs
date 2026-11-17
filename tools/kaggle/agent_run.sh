#!/bin/bash
# TEMPORARY (agent sb02): print every row verdict for SCOREBOARD.md. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
bash tools/oracle/run_all.sh 2>&1 | grep -E 'PASS|FAIL|SKIP|PANIC'
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1
