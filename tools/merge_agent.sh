#!/bin/bash
# Merge finished agent branches into main, keeping main compiling.
# Usage: tools/merge_agent.sh <agent-id | branch-name> ...
# Commits leftover changes in the agent's worktree, merges, runs cargo check (own target dir).
# Build OK -> keep merge. Build broken -> save the merged state as branch fix-<id> (for a fixer agent),
# reset main. Worktrees are NOT removed here (an agent may still be finishing); use tools/cleanup_worktrees.sh.
cd "$(dirname "$0")/.."
LOCK=/c/Users/karti/sumo-rs-merge.lock
until mkdir "$LOCK" 2>/dev/null; do sleep 5; done   # one merge run at a time; manual commits to main must take it too
trap 'rmdir "$LOCK"' EXIT
export PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-main CARGO_BUILD_JOBS=8
for id in "$@"; do
  case "$id" in worktree-agent-*|fix-*|local-fill) br=$id; wt="" ;; *) br=worktree-agent-$id; wt=.claude/worktrees/agent-$id ;; esac
  if [ -n "$wt" ] && [ -d "$wt" ] && [ -n "$(git -C "$wt" status --porcelain -- crates)" ]; then
    git -C "$wt" add -A crates && git -C "$wt" commit -qm "Leftover edits (committed on agent's behalf)" -m "Co-Authored-By: Claude Haiku 5.5 <noreply@anthropic.com>"
  fi
  [ -z "$(git log --oneline main.."$br" 2>/dev/null)" ] && { echo "$id: nothing new"; continue; }
  before=$(git rev-parse HEAD)
  if ! git merge -q --no-edit "$br" >/dev/null 2>&1; then
    git checkout -q --theirs Cargo.lock 2>/dev/null; git add Cargo.lock 2>/dev/null
    if [ -n "$(git diff --name-only --diff-filter=U)" ]; then
      echo "$id: CONFLICT in $(git diff --name-only --diff-filter=U | tr '\n' ' ') - aborted"; git merge --abort; continue
    fi
    git commit -qm "Merge $br" --no-edit
  fi
  errs=$(cargo check -q -p sumo_core --message-format short 2>&1 | grep -c ': error')
  if [ "$errs" != "0" ]; then
    name=fix-${id#worktree-agent-}; git branch -f "$name" HEAD; git reset -q --hard "$before"
    echo "$id: $errs build errors -> saved as $name for a fixer"; continue
  fi
  echo "$id: merged OK, main has $(grep -rc 'todo!(' crates/sumo_core/src | awk -F: '{s+=$2} END {print s}') todo!() left"
  git push -q
done
