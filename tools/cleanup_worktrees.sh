#!/bin/bash
# Remove agent worktrees/branches already fully merged into main.
cd "$(dirname "$0")/.."
for br in $(git branch --format='%(refname:short)' | grep -E '^(worktree-agent-|fix-)'); do
  if [ -z "$(git log --oneline main.."$br")" ]; then
    id=${br#worktree-agent-}; git worktree remove --force ".claude/worktrees/agent-$id" 2>/dev/null; git branch -D -q "$br" && echo "removed $br"
  fi
done
