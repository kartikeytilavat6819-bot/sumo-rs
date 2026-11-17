#!/bin/bash
# ub02 (temporary, removed in the final commit): release build and run_all, to compare release verdicts with debug.
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_BUILD_JOBS=8
REL=${CARGO_TARGET_DIR}-rel
echo "== release build ($REL)"
CARGO_TARGET_DIR=$REL cargo build --release -p sumo -p netconvert --message-format short > /tmp/ub02_rel_build.log 2>&1
rc=$?
echo "release build rc=$rc"
grep -aE ': error|^error|panicked' /tmp/ub02_rel_build.log | head -20
if [ $rc -ne 0 ]; then echo "RELEASE BUILD FAILED"; exit 0; fi
(cd "$REL/release" && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)
echo "== release run_all"
PROFILE=release CARGO_TARGET_DIR=$REL bash tools/oracle/run_all.sh > /tmp/ub02_rel_run.log 2>&1
grep -aE 'PASS|FAIL|SKIP|PANIC|STALE|MISSING' /tmp/ub02_rel_run.log
echo "AGENT_RUN END"
