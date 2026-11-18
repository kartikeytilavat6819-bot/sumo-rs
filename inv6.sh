#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
sed -n '336,390p' crates/sumo_core/src/microsim/ms_vehicle_transfer.rs
