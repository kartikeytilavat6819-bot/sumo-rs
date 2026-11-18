#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc54 || exit 1
rm -f tools/kaggle/agent_run.sh
git add -A
git commit -m "oc54: park 21 multi-modal rows, remove agent_run.sh" -q
git log --oneline -1
