#!/bin/bash
echo "=== C++ allowedLanes declarations ==="
grep -n "allowedLanes" /c/Users/karti/sumo-src/src/netbuild/MSEdge.h /c/Users/karti/sumo-src/src/microsim/MSEdge.h 2>/dev/null | head -20
echo "=== Rust allowed_lanes / get_free_lane ==="
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
grep -rn "pub fn allowed_lanes\|pub fn get_free_lane" crates/sumo_core/src/microsim/ms_edge.rs | head -20
