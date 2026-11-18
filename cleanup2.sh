#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
git rm -f commit1.sh >/dev/null 2>&1
rm -f checkcommit.sh
git add -A
git commit -m "oc55: remove scratch script" 2>&1 | tail -3
echo "=== status ==="
git status --short
git log --oneline -3
