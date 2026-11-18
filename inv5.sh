#!/bin/bash
cd /c/Users/karti/sumo-rs/.claude/worktrees/agent-oc55 || exit
echo "=== C++ around 235-255 ==="
sed -n '235,255p' /c/Users/karti/sumo-src/src/microsim/MSVehicleTransfer.cpp
echo "=== Rust 395-425 ==="
sed -n '395,425p' crates/sumo_core/src/microsim/ms_vehicle_transfer.rs
