#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
echo "=== Rust 238-340 ==="
sed -n '238,340p' crates/sumo_core/src/microsim/ms_vehicle_transfer.rs
