#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
rm -f verify.sh
git add -A
git commit -m "oc55: import extended chunk 1 (68 cases) and fix importer nested/additional files" 2>&1 | tail -8
echo "=== after ==="
git status --short | head -5
git log --oneline -2
