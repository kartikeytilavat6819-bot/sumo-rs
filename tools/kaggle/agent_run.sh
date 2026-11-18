#!/bin/bash
# oc22 gate runner: random-scenario fuzzing over NEW seed ranges on the VM gate.
# Runs ~30 min total (two FUZZ_BUDGET=420 ranges + tool overhead), prints only failures/shrinks.
set -u
cd "$(dirname "$0")/../.." || exit 1
export FUZZ_OUT=/tmp/fuzz_oc22
LOG=/tmp/fuzz_oc22_gate.log
: > "$LOG"
for FIRST in 3000 4000; do
  echo "### fuzz range starting at $FIRST" | tee -a "$LOG"
  FUZZ_FIRST=$FIRST FUZZ_BUDGET=420 bash tools/oracle/fuzz_sim.sh 45 >>"$LOG" 2>&1
done
echo "### FAIL/PANIC lines" 
grep -E '^seed .* => (FAIL|PANIC)' "$LOG" | head -60
echo "### signatures / budget"
grep -E '^(=== |  sig |budget:|--- shrink|  shrunk:|GENFAIL|REFFAIL)' "$LOG" | head -80
