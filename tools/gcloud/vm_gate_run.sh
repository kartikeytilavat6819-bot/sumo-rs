#!/bin/bash
# Runs on the gate VM as user gate: gate one snapshot (same steps as tools/kaggle/batch_kernel.py) into stdout.
# Usage: vm_gate_run.sh <snapshot.tgz> "<branch> <sha>"
set -u
# 6 slots per VM (run_all is single-threaded, so one gate leaves 15 cores idle; 62 GB RAM); each slot has its own tree + target
slot=""
while [ -z "$slot" ]; do
  for s in 0 1 2 3 4 5; do exec 9>/home/gate/slot$s.lock; if flock -n 9; then slot=$s; break; fi; exec 9>&-; done
  [ -z "$slot" ] && sleep 5
done
SRC=/home/gate/s$slot/src NEW=/home/gate/s$slot/new
export SLOT_TARGET=/home/gate/s$slot/target
rm -rf "$NEW"; mkdir -p "$NEW" "$SRC"; tar -xzf "$1" -C "$NEW"
find "$NEW/tools" -type f \( -name '*.sh' -o -name '*.args' \) -exec sed -i 's/\r$//' {} +
# no -t: changed files get a fresh mtime so cargo rebuilds them (git archive stamps the commit time)
echo "$2"; echo "files changed: $(rsync -rc --delete --out-format='%n' "$NEW/" "$SRC/" | grep -vc '/$')"
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$SLOT_TARGET CARGO_BUILD_JOBS=8 SCORE_OUT=/tmp/score$slot \
       SUMO_HOME=/home/gate/sumo_pkg/sumo SUMO_PRISTINE_BIN=/home/gate/sumo_pkg/sumo/bin
cd "$SRC"
cargo build -p sumo -p netconvert --message-format short > /tmp/build$slot.log 2>&1; rc=$?
# a stale incremental cache can crash rustc (ICE): drop it and build once more
if [ $rc -ne 0 ] && grep -aq 'internal compiler error' /tmp/build$slot.log; then
  echo "ICE: clearing slot $slot incremental cache"; rm -rf "$CARGO_TARGET_DIR/debug/incremental"
  cargo build -p sumo -p netconvert --message-format short > /tmp/build$slot.log 2>&1; rc=$?
fi
grep -aE ': error|^error' /tmp/build$slot.log | head -40
# a failed build leaves the previous snapshot's binary behind: never test it
if [ $rc -ne 0 ]; then echo "BUILD FAILED"; exit 1; fi
(cd "$CARGO_TARGET_DIR/debug" && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)
bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING'
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1
cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10
echo "GATE DONE"
# agents write /tmp/target (the Kaggle path); here every slot has its own target
if [ -f tools/kaggle/agent_run.sh ]; then sed -i "s#/tmp/target#$CARGO_TARGET_DIR#g" tools/kaggle/agent_run.sh; timeout 600 bash tools/kaggle/agent_run.sh 2>&1 | tail -400; echo "AGENT RUN DONE"; fi
