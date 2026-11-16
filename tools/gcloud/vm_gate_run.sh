#!/bin/bash
# Runs on the gate VM as user gate: gate one snapshot (same steps as tools/kaggle/batch_kernel.py) into stdout.
# Usage: vm_gate_run.sh <snapshot.tgz> "<branch> <sha>"
set -u
exec 9>/home/gate/gate.lock; flock 9          # one gate at a time; the target dir is shared and stays warm
SRC=/home/gate/src NEW=/home/gate/new
rm -rf "$NEW"; mkdir -p "$NEW" "$SRC"; tar -xzf "$1" -C "$NEW"
find "$NEW/tools" -type f \( -name '*.sh' -o -name '*.args' \) -exec sed -i 's/\r$//' {} +
# no -t: changed files get a fresh mtime so cargo rebuilds them (git archive stamps the commit time)
echo "$2"; echo "files changed: $(rsync -rc --delete --out-format='%n' "$NEW/" "$SRC/" | grep -vc '/$')"
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=/home/gate/target CARGO_BUILD_JOBS=16 SCORE_OUT=/tmp/score \
       SUMO_HOME=/home/gate/sumo_pkg/sumo SUMO_PRISTINE_BIN=/home/gate/sumo_pkg/sumo/bin
cd "$SRC"
cargo build -p sumo -p netconvert --message-format short > /tmp/build.log 2>&1; rc=$?
grep -E ': error|^error' /tmp/build.log | head -40
# a failed build leaves the previous snapshot's binary behind: never test it
if [ $rc -ne 0 ]; then echo "BUILD FAILED"; exit 1; fi
(cd "$CARGO_TARGET_DIR/debug" && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)
bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|PASS=|STALE|MISSING'
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1
cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10
echo "GATE DONE"
if [ -f tools/kaggle/agent_run.sh ]; then timeout 600 bash tools/kaggle/agent_run.sh 2>&1 | tail -400; echo "AGENT RUN DONE"; fi
