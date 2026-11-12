#!/bin/bash
# Unattended fill of skeleton todo!() bodies with OpenCode's FREE models (via the opencode CLI; the raw
# API refuses free models). Same loop as overnight_local.sh minus vLLM: fill 15 files per round, merge
# each round into main (compile-checked, pushed), requeue files whose merge conflicts.
# Log: C:\Users\karti\sumo-rs-opencode.log    Launch: see bottom of PORTING.md
LOG=/c/Users/karti/sumo-rs-opencode.log
WT=/c/Users/karti/sumo-rs-local
REPO=/c/Users/karti/sumo-rs
STATE=/c/Users/karti/sumo-rs-opencode.state
export LLM_BACKEND=opencode
log() { echo "$(date '+%F %T') $*" >> "$LOG"; }

powershell.exe -NoProfile -Command '$t=Add-Type -MemberDefinition "[DllImport(\"kernel32.dll\")] public static extern uint SetThreadExecutionState(uint f);" -Name W -Namespace K -PassThru; while($true){ $t::SetThreadExecutionState(0x80000001) | Out-Null; Start-Sleep 30 }' &
AWAKE=$!
trap 'kill $AWAKE 2>/dev/null' EXIT
mkdir -p /c/Users/karti/oc-sandbox
log "start (keep-awake pid $AWAKE)"

# skip files that unmerged Haiku branches touched (their own commits + uncommitted edits), so merging those later is clean
exclude() {
  cd "$REPO" || return
  { for b in $(git branch --no-merged main --format='%(refname:short)' | grep -v '^local-fill$'); do
      git log main.."$b" --name-only --format= ; done
    for w in .claude/worktrees/*/; do git -C "$w" status --porcelain 2>/dev/null | awk '{print $2}'; done
  } | grep '\.rs$' | xargs -n1 basename 2>/dev/null | sort -u | paste -sd, -
}

cd "$WT" || { log "worktree $WT missing"; exit 1; }
for round in $(seq 1 1000); do
  git -C "$WT" merge -q --no-edit main >> "$LOG" 2>&1 || { git -C "$WT" merge --abort 2>/dev/null; git -C "$WT" reset -q --hard main; }
  EXCLUDE=$(exclude)
  OUT=$(python "$REPO/tools/local_fill.py" --worktree "$WT" --max-lines 40 --limit-files 15 \
          --state "$STATE" --exclude "$EXCLUDE" 2>&1)
  echo "$OUT" | grep -v '^$' >> "$LOG"
  if echo "$OUT" | grep -q "all OpenCode free models failed"; then log "free usage limit hit; waiting 30 min"; sleep 1800; fi
  log "round $round done; merging into main"
  RES=$(bash "$REPO/tools/merge_agent.sh" local-fill 2>&1); echo "$RES" | grep -v '^warning' >> "$LOG"
  if echo "$RES" | grep -qE "build errors|CONFLICT"; then
    for f in $(git -C "$WT" log main..local-fill --name-only --format= | sort -u); do
      grep -vxF "$f" "$STATE" > "$STATE.tmp" 2>/dev/null; mv "$STATE.tmp" "$STATE"
    done
    git -C "$WT" reset -q --hard main; git -C "$REPO" branch -D -q fix-local-fill 2>/dev/null
    log "merge failed; reset local-fill to main, files queued again"
  fi
  echo "$OUT" | grep -q "REMAINING_FILES=0" && { log "all files processed"; break; }
done
log "opencode fill finished; $(python "$REPO/tools/progress.py" | tail -1)"
