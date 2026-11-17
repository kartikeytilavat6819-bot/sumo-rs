#!/bin/bash
# Temporary (sl04): state/load parity for every sim case. Removed before merge.
cd "$(dirname "$0")/../.." || exit 1
export STATE_OUT=/tmp/state_sl04 JOBS=16
bash tools/oracle/run_state.sh > /tmp/state_sl04_run.txt 2>&1
grep -E 'variants=|state=(FAIL|NOSTATE)|load=(FAIL|NOLOAD)|NONE' /tmp/state_sl04_run.txt | cut -c1-200 | head -80
for f in $(find /tmp/state_sl04 -name '*.ours.state.xml' | sort); do
  p=${f%.ours.state.xml}.pristine.state.xml
  [ -f "$p" ] || continue
  if ! cmp -s "$f" "$p"; then
    echo "STATE-IDS $(echo "$f" | sed 's#.*/state-[0-9]*/##')"
    comm -3 <(grep -o '<vehicle id="[^"]*"' "$p" | sort) <(grep -o '<vehicle id="[^"]*"' "$f" | sort) | head -6 | sed 's/^/   /'
    echo "   counts: ref_vehicles=$(grep -c '<vehicle ' "$p") ours_vehicles=$(grep -c '<vehicle ' "$f")"
  fi
done
for f in $(find /tmp/state_sl04 -name '*.save_ours.log' | sort); do
  grep -q 'panicked\|Error' "$f" && echo "SAVE-LOG $(echo "$f" | sed 's#.*/state-[0-9]*/##'): $(grep -m1 -A1 'panicked\|Error' "$f" | tr '\n' ' ' | cut -c1-220)"
done
grep -rh -A1 'panicked at' /tmp/state_sl04 --include=*.log 2>/dev/null | grep -v minGapLat | cut -c1-200 | sort | uniq -c | sort -rn | head -20
