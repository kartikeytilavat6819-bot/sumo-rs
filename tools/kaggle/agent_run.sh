#!/bin/bash
# TEMPORARY (nb03): netopts (188 variants) C++ 1.28.0 vs ours after the netbuild shortcut port. Removed before merge.
set -u
export RUST_NETCONVERT=$CARGO_TARGET_DIR/debug/netconvert
export CPP_NETCONVERT=$SUMO_PRISTINE_BIN/netconvert
export CPP_SUMO_HOME=${SUMO_HOME:-/home/gate/sumo_pkg/sumo}
export NETOPTS_OUT=/tmp/nb03_netopts
rm -rf $NETOPTS_OUT
bash tools/oracle/run_netopts.sh > /tmp/nb03_netopts.log 2>&1
echo "netopts rc=$?"
grep -h 'TOTAL' /tmp/nb03_netopts.log
cat $NETOPTS_OUT/*/summary.tsv | grep -v -P '\tPASS\t' | head -40
f=$(ls $NETOPTS_OUT/real/*/cpp.log 2>/dev/null | head -1)
[ -n "$f" ] && { echo "sample cpp.log: $f"; head -5 "$f"; }
echo "NB03 NETOPTS END"
