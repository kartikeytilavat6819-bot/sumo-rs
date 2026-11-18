#!/bin/bash
# Runs on the gate VM as user gate: gate one snapshot (same steps as tools/kaggle/batch_kernel.py) into stdout.
# Usage: vm_gate_run.sh <snapshot.tgz> "<branch> <sha>"
set -u
# 8 slots per VM (run_all is single-threaded, so one gate leaves 15 cores idle; 62 GB RAM); each slot has its own tree + target
slot=""
while [ -z "$slot" ]; do
  for s in 0 1 2 3 4 5 6 7; do exec 9>/home/gate/slot$s.lock; if flock -n 9; then slot=$s; break; fi; exec 9>&-; done
  [ -z "$slot" ] && sleep 5
done
SRC=/home/gate/s$slot/src NEW=/home/gate/s$slot/new
export SLOT_TARGET=/home/gate/s$slot/target
rm -rf "$NEW"; mkdir -p "$NEW" "$SRC"; tar -xzf "$1" -C "$NEW" || { echo "SNAPSHOT CORRUPT (truncated upload)"; exit 3; }  # no GATE DONE: vm_gate.ps1 retries the upload
find "$NEW/tools" -type f \( -name '*.sh' -o -name '*.args' \) -exec sed -i 's/\r$//' {} +
# no -t: changed files get a fresh mtime so cargo rebuilds them (git archive stamps the commit time)
echo "$2"; echo "files changed: $(rsync -rc --delete --out-format='%n' "$NEW/" "$SRC/" | grep -vc '/$')"
export PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR=$SLOT_TARGET CARGO_BUILD_JOBS=8 SCORE_OUT=/tmp/score$slot \
       SUMO_HOME=/home/gate/sumo_pkg/sumo SUMO_PRISTINE_BIN=/home/gate/sumo_pkg/sumo/bin
cd "$SRC"
# run_all writes a new run-$$ dir per run; the slot is locked, so its old results can go (they filled the disk)
rm -rf "$SCORE_OUT"; find /tmp -maxdepth 1 -name '*.tgz' -mmin +60 -delete 2>/dev/null
PKGS="-p sumo -p netconvert"; for t in duarouter dfrouter activitygen od2trips marouter jtrrouter netgenerate polyconvert; do [ -d crates/$t ] && PKGS="$PKGS -p $t"; done
cargo build $PKGS --message-format short > /tmp/build$slot.log 2>&1; rc=$?
# a stale incremental cache can crash rustc (ICE): drop it and build once more
if [ $rc -ne 0 ] && grep -aqE 'internal compiler error|incremental' /tmp/build$slot.log; then
  echo "ICE: clearing slot $slot incremental cache"; rm -rf "$CARGO_TARGET_DIR/debug/incremental"
  cargo build $PKGS --message-format short > /tmp/build$slot.log 2>&1; rc=$?
fi
grep -aE ': error|^error' /tmp/build$slot.log | head -40
# a failed build leaves the previous snapshot's binary behind: never test it
if [ $rc -ne 0 ]; then echo "BUILD FAILED"; exit 1; fi
(cd "$CARGO_TARGET_DIR/debug" && ln -sf sumo sumo.exe && ln -sf netconvert netconvert.exe)
bash tools/oracle/run_all.sh 2>&1 | grep -E 'FAIL|PANIC|TIMEOUT|PASS=|STALE|MISSING'
bash tools/oracle/cases/depart/run_all_variants.sh 2>&1 | tail -1
if [ -f tools/oracle/run_duarouter.sh ] && [ -x "$CARGO_TARGET_DIR/debug/duarouter" ]; then
  OUT=/tmp/dua$slot timeout 600 bash tools/oracle/run_duarouter.sh >/dev/null 2>&1
  echo "duarouter oracle: PASS=$(grep -c $'\tPASS\t' /tmp/dua$slot/results.tsv) of $(($(wc -l < /tmp/dua$slot/results.tsv) - 1))"; grep -v $'\tPASS\t' /tmp/dua$slot/results.tsv | tail -n +2 | cut -f1,2,5 | head -10
fi
# netconvert option rows outside run_all (h08); one summary line + the non-PASS row names
if [ -f tools/oracle/run_netopts_new.sh ]; then
  rm -rf /tmp/netopts2_$slot; NETOPTS2_OUT=/tmp/netopts2_$slot timeout 900 bash tools/oracle/run_netopts_new.sh >/dev/null 2>&1
  T=/tmp/netopts2_$slot/summary.tsv
  echo "netopts_new: PASS=$(grep -c $'\tPASS\t' $T 2>/dev/null) of $(cat $T 2>/dev/null | wc -l); not passing: $(grep -v $'\tPASS\t' $T 2>/dev/null | cut -f1,2 | tr '\t\n' ': ')"
fi
for t in dfrouter activitygen; do
  if [ -f tools/oracle/run_dfag.sh ] && [ -x "$CARGO_TARGET_DIR/debug/$t" ]; then
    TOOL=$t OUT=/tmp/${t}_o$slot timeout 600 bash tools/oracle/run_dfag.sh >/dev/null 2>&1
    echo "$t oracle: PASS=$(grep -c $'	PASS	' /tmp/${t}_o$slot/results.tsv) of $(($(wc -l < /tmp/${t}_o$slot/results.tsv) - 1))"; grep -v $'	PASS	' /tmp/${t}_o$slot/results.tsv | tail -n +2 | cut -f1,2,5 | head -10
  fi
done
for t in od2trips marouter; do
  if [ -x "$CARGO_TARGET_DIR/debug/$t" ] && [ -x "$SUMO_PRISTINE_BIN/$t" ]; then
    OUT=/tmp/${t}$slot timeout 600 bash tools/oracle/run_odtool.sh $t >/dev/null 2>&1
    echo "$t oracle: PASS=$(grep -c $'	PASS	' /tmp/${t}$slot/results.tsv) of $(($(wc -l < /tmp/${t}$slot/results.tsv) - 1))"; grep -v $'	PASS	' /tmp/${t}$slot/results.tsv | tail -n +2 | cut -f1,2,5 | head -10
  else echo "$t oracle: SKIPPED (missing our binary or $SUMO_PRISTINE_BIN/$t)"; fi
done
# jtrrouter oracle (block z30)
if [ -f tools/oracle/run_jtrrouter.sh ] && [ -x "$CARGO_TARGET_DIR/debug/jtrrouter" ]; then
  OUT=/tmp/jtr$slot timeout 600 bash tools/oracle/run_jtrrouter.sh >/dev/null 2>&1
  echo "jtrrouter oracle: PASS=$(grep -c $'	PASS	' /tmp/jtr$slot/results.tsv) of $(($(wc -l < /tmp/jtr$slot/results.tsv) - 1))"; grep -v $'	PASS	' /tmp/jtr$slot/results.tsv | tail -n +2 | cut -f1,2,5 | head -12
fi
# netgenerate / polyconvert oracles (tools/oracle/run_gen_poly.sh)
[ -f tools/oracle/run_gen_poly.sh ] && GENPOLY_OUT=/tmp/genpoly$slot timeout 900 bash tools/oracle/run_gen_poly.sh 2>&1
# complex TraCI suite oracle: official vs our sumo over the bundled runner.py scripts
if [ -f tools/oracle/run_complex_suite.sh ]; then
  COMPLEX_OUT=/tmp/complex$slot COMPLEX_TIMEOUT=12 timeout 600 bash tools/oracle/run_complex_suite.sh > /tmp/complexlog$slot 2>&1
  grep '^complex suite oracle:' /tmp/complexlog$slot
  grep '^FAIL' /tmp/complexlog$slot | head -12
fi
cargo test -q -p sumo_core -p utils 2>&1 | grep -E 'test result|FAILED|panicked|error' | head -10
echo "GATE DONE"
# agents write /tmp/target (the Kaggle path); here every slot has its own target
if [ -f tools/kaggle/agent_run.sh ]; then sed -i "s#/tmp/target#$CARGO_TARGET_DIR#g" tools/kaggle/agent_run.sh; timeout 1800 bash tools/kaggle/agent_run.sh 2>&1 | tail -400; echo "AGENT RUN DONE"; fi
