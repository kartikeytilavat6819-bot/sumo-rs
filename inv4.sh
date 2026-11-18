#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
echo "=== C++ myJumping ==="
grep -n "myJumping" /c/Users/karti/sumo-src/src/microsim/MSVehicleTransfer.cpp
echo "=== Rust myJumping ==="
grep -n "myJumping" crates/sumo_core/src/microsim/ms_vehicle_transfer.rs | head -30
