#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/wave2 || exit 1
for b in "$@"; do
  if git merge -q --no-edit "$b" >/tmp/m.log 2>&1; then
    echo "$b merged"
  else
    echo "$b CONFLICT"
    grep -i conflict /tmp/m.log | head -8
    exit 1
  fi
done
git log --oneline -1
git status --short | head -5
