#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
sed -n '2321,2345p' crates/sumo_core/src/microsim/ms_edge.rs
