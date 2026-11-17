#!/bin/bash
# TEMPORARY (nx04): netopts matrix on the VM, ours vs official C++. Removed before the final commit.
set -u
export RUST_NETCONVERT=$CARGO_TARGET_DIR/debug/netconvert
export CPP_NETCONVERT=$SUMO_PRISTINE_BIN/netconvert
export CPP_SUMO_HOME=$SUMO_HOME
export NETOPTS_OUT=/tmp/nx04-netopts
export NETOPTS_JOBS=4
rm -rf $NETOPTS_OUT
bash tools/oracle/run_netopts.sh > /tmp/nx04-netopts.log 2>&1
echo "netopts rc=$?"
grep -a 'TOTAL' /tmp/nx04-netopts.log
echo "non-PASS rows:"
cat $NETOPTS_OUT/*/summary.tsv | grep -av -P '\tPASS\t' | cut -c1-300
echo "NX04 NETOPTS END"
