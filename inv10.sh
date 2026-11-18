#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
grep -rn "allowed_lanes(.*)\.m()\|allowed_lanes.*\.m()" crates/sumo_core/src/ | head -20
