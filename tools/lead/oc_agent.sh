#!/bin/bash
# Lead driver for one OpenCode agent on a FREE model (user's order: OpenCode CLI, free models only).
# OpenCode edits + commits in its own worktree; after each round the lead's VM gate checks the branch and the
# result (with regressions vs main's known FAILs) goes back into the same OpenCode session.
#   bash tools/lead/oc_agent.sh <ID> <task-file> [max-rounds]     (run in background)
# Progress: .claude/oc-<ID>.log (short), .claude/oc-<ID>.log.json (raw OpenCode events), .claude/oc-<ID>.lastgate
set -u
ID=$1 TASK=$2 ROUNDS=${3:-10}
REPO=/c/Users/karti/sumo-rs WT=$REPO/.claude/worktrees/agent-$ID BR=worktree-agent-$ID
OC="$APPDATA/npm/node_modules/@opencode/cli/bin/opencode.exe"
MODEL=${OC_MODEL:-opencode/hy4-flash-free}
LOG=$REPO/.claude/oc-$ID.log
# rows failing on main 5413c53b: not regressions
KNOWN=" out_bt/ref out_meandata/ref out_tls/ref elechybrid_overhead/ref emis_mmpevem_params/ref lc_lc2013_turnlanes_dur/ref meso_calibrator_edgedata/ref state_meso/ref taxi_greedy_shared/ref taxi_route_extension/ref duarouter_persontrip_walkfactor duarouter_intermodal_outputs opendriveOutput "

say() { echo "$(date +%H:%M) $*" | tee -a "$LOG"; }
# every failing line whose row is not a known main FAIL, plus build/test/harness failures
regressions() {
  local r=""
  grep -q 'GATE DONE' <<<"$1" || r="gate did not finish;"
  while read -r row _; do
    [[ "$KNOWN" == *" $row "* ]] || r="$r $row;"
  done < <(grep -E 'FAIL|PANIC|TIMEOUT|panicked|STALE|MISSING' <<<"$1" | grep -vE '(^| )PASS=[0-9]+ FAIL=0|^PASS=')
  # the run_all summary "PASS=n FAIL=m" is judged by the rows above, not by its count
  echo "$r"
}
last_text() { python -c "
import sys, json
t = ''
for l in sys.stdin:
    try: p = (json.loads(l).get('part') or {})
    except Exception: continue
    if p.get('type') == 'text': t = p.get('text', '')
print(t[-1500:])"; }

[ -d "$WT" ] || git -C "$REPO" worktree add -q "$WT" -b "$BR" "${OC_BASE:-main}"
cp "$REPO/tools/lead/oc_permissions.jsonc" "$WT/opencode.jsonc"
msg="$(sed "s/<ID>/$ID/g" "$REPO/tools/lead/${OC_RULES:-oc_rules.md}"; echo; cat "$TASK")"
# OC_BASELINE=1 (branches not based on main): gate the untouched base once; whatever fails there is not a regression
if [ -n "${OC_BASELINE:-}" ]; then
  say "baseline gate of $(git -C "$WT" rev-parse --short HEAD)"
  base=$(powershell.exe -ExecutionPolicy Bypass -File 'C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1' -Branch "$BR" 2>&1 | tr -d '\r')
  KNOWN="$KNOWN $(grep -E 'FAIL|PANIC|TIMEOUT|panicked|STALE|MISSING' <<<"$base" | grep -vE '^PASS=' | awk '{print $1}' | tr '\n' ' ') "
  say "baseline known rows:$KNOWN"
fi
sid=""
for r in $(seq 1 "$ROUNDS"); do
  before=$(git -C "$WT" rev-parse HEAD)
  say "round $r: $MODEL working${sid:+ (session $sid)}"
  out=$(cd "$WT" && printf '%s' "$msg" | timeout 3600 "$OC" run --auto -m "$MODEL" --format json --archived ${sid:+--session "$sid"} 2>&1)
  printf '%s\n' "$out" >> "$LOG.json"
  [ -n "$sid" ] || sid=$(grep -o '"sessionID":"ses_[A-Za-z0-9]*"' <<<"$out" | head -1 | cut -d'"' -f4)
  if grep -q 'Free usage limit reached' <<<"$out"; then
    # user's order: use OpenCode only until the free limit is used up, then stop
    say "free usage limit reached: stopping"; break
  fi
  say "reply: $(last_text <<<"$out" | tr '\n' ' ' | cut -c1-600)"
  if [ -n "$(git -C "$WT" status --porcelain)" ]; then
    msg="You left uncommitted changes. Commit them (git add -A && git commit) and reply with your summary."; continue
  fi
  after=$(git -C "$WT" rev-parse HEAD)
  if [ "$before" = "$after" ]; then
    # the free model sometimes ends its turn mid-task ("Let me verify..."): nudge it twice before giving up
    if ! last_text <<<"$out" | grep -q 'DONE' && [ "${nudges:-0}" -lt 2 ]; then
      nudges=$(( ${nudges:-0} + 1 )); say "no commit yet: nudging ($nudges)"
      msg="Continue your task now: do the work (edit files, generate references, commit with git add -A && git commit). Do not stop to announce what you will do next; stop only after a commit, with a short summary, or reply DONE."
      continue
    fi
    say "no new commit: stopping"; break
  fi
  nudges=0
  sha=$(git -C "$WT" rev-parse --short HEAD)
  say "gating $sha"
  res=$(powershell.exe -ExecutionPolicy Bypass -File 'C:\Users\karti\sumo-rs\tools\gcloud\vm_gate.ps1' -Branch "$BR" 2>&1 | tr -d '\r')
  printf '%s\n' "$res" > "$REPO/.claude/oc-$ID.lastgate"
  reg=$(regressions "$res")
  say "gate $sha: $(grep -E '^PASS=|depart variants|duarouter oracle|BUILD FAILED' <<<"$res" | tr '\n' ' ') | regressions: ${reg:-none}"
  msg="Gate result for your commit $sha:
$res

Regressions (rows that pass on main but not on your branch, or build/test failures): ${reg:-none}.
If BUILD FAILED: fix the compile errors shown above (they are in your files). If a previously passing row now fails:
find which of your changes caused it and fix it faithfully, or undo that change. Commit, then continue with the next
10-15 shortcuts in your area and reply with your summary. If nothing more in your area can be ported faithfully, reply DONE with a final summary."
done
say "finished after round $r; branch $BR at $(git -C "$WT" rev-parse --short HEAD)"
