#!/bin/bash
# Temporary (sl04): state/load parity for every sim case. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/state_sl04 JOBS=16
bash tools/oracle/run_state.sh > /tmp/state_sl04_run.txt 2>&1
grep -E 'state=(FAIL|NOSTATE)|load=(FAIL|NOLOAD)|SKIP|variants=|state /|load /' /tmp/state_sl04_run.txt | head -150
grep -rhE 'panicked' /tmp/state_sl04 --include=*.log 2>/dev/null | cut -c1-240 | sort | uniq -c | sort -rn | head -40
