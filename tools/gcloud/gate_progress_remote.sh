#!/bin/bash
# Run as root on a gate VM by gate_progress.ps1 (read-only). One line per gate run:
#   slot|branch sha|elapsed s|step|cases done|cases total|detail      plus  DISK|use%
ps -eo pid=,ppid=,etimes=,args= > /tmp/.gp$$
for pid in $(pgrep -f '^bash /home/gate/run_'); do
  # the run script holds its slot lock on fd 9 (vm_gate_run.sh)
  slot=$(readlink /proc/$pid/fd/9 2>/dev/null | sed -n 's#.*/slot\([0-9]*\)\.lock$#\1#p')
  label=$(tr '\0' '\n' < /proc/$pid/cmdline | tail -1)
  et=$(awk -v p=$pid '$1==p{print $3}' /tmp/.gp$$)
  kids=$(awk -v r=$pid '{pp[$1]=$2; l[$1]=$0} END{d[r]=1; do {ch=0; for (p in pp) if (!(p in d) && (pp[p] in d)) {d[p]=1; ch=1}} while (ch); for (p in d) if (p!=r) print l[p]}' /tmp/.gp$$)
  has() { printf '%s\n' "$kids" | grep -qF "$1"; }
  done=0; total=0; detail=""; step=waiting
  if [ -z "$slot" ]; then step="waiting for a free slot"
  elif has agent_run.sh; then step=agent_run
  elif has 'cargo test'; then step="unit tests"
  elif has run_duarouter; then step=duarouter
  elif has run_all_variants; then step=depart
  elif has run_all.sh; then
    step=run_all; cases=$(ls /home/gate/s$slot/src/tools/oracle/cases)
    total=$(printf '%s\n' "$cases" | wc -l)
    last=$(ls -t /tmp/score$slot/run-*/ 2>/dev/null | head -1); detail=${last%%.*}
    [ -n "$detail" ] && done=$(printf '%s\n' "$cases" | grep -nxF "$detail" | cut -d: -f1)
  elif has 'cargo build'; then step=build; detail=$(grep -a 'Compiling' /tmp/build$slot.log 2>/dev/null | tail -1 | sed 's/^ *//')
  fi
  echo "$slot|$label|$et|$step|${done:-0}|$total|$detail"
done
rm -f /tmp/.gp$$
echo "DISK|$(df --output=pcent /home | tail -1 | tr -d ' ')"
