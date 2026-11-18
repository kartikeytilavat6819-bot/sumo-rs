#!/bin/bash
# oc22 gate runner (temporary): random-scenario fuzzing over NEW seed ranges on the VM gate.
set -u
cd "$(dirname "$0")/../.." || exit 1
export FUZZ_OUT=/tmp/fuzz_oc22_r3
LOG=/tmp/fuzz_oc22_r3.log
: > "$LOG"
for FIRST in 5000 6000; do
  echo "### fuzz range starting at $FIRST" | tee -a "$LOG"
  FUZZ_FIRST=$FIRST FUZZ_BUDGET=420 bash tools/oracle/fuzz_sim.sh 45 >>"$LOG" 2>&1
done
echo "### FAIL/PANIC lines"
grep -E '^seed .* => (FAIL|PANIC)' "$LOG" | head -60
echo "### signatures / budget"
grep -E '^(=== |  sig |budget:|--- shrink|  shrunk:|GENFAIL|REFFAIL)' "$LOG" | head -80
