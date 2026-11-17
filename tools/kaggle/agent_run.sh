#!/bin/bash
# Temporary (sl04): state/load parity for every sim case. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/state_sl04 JOBS=16
bash tools/oracle/run_state.sh > /tmp/state_sl04_run.txt 2>&1
grep -E 'variants=|state=(FAIL|NOSTATE)|load=(FAIL|NOLOAD)|NONE' /tmp/state_sl04_run.txt | cut -c1-200 | head -80
grep -E '^      (state|load) ' /tmp/state_sl04_run.txt | cut -c1-260 | head -60
grep -rh -A1 'panicked at' /tmp/state_sl04 --include=*.log 2>/dev/null | cut -c1-260 | sort | uniq -c | sort -rn | head -30
for f in $(grep -rl 'panicked at' /tmp/state_sl04 --include=*.log 2>/dev/null | head -12); do echo "$f" | sed 's#.*/state-[0-9]*/##'; done
