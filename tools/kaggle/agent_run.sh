#!/bin/bash
# the gate on the VM uses main's vm_gate_run.sh, so build + run the new oracles here
export PATH=$HOME/.cargo/bin:$PATH CARGO_BUILD_JOBS=8
cargo build -p netgenerate -p polyconvert --message-format short > /tmp/z32build.log 2>&1; rc=$?
grep -aE 'error' /tmp/z32build.log | head -40
[ $rc -ne 0 ] && { echo "Z32 BUILD FAILED"; exit 1; }
GENPOLY_OUT=/tmp/z32genpoly timeout 1200 bash tools/oracle/run_gen_poly.sh 2>&1 | cut -c1-400
for f in /tmp/z32genpoly/pc/xmlShapes/rust.log /tmp/z32genpoly/pc/xmlShapes/cpp.log; do echo "== $f"; head -15 $f | cut -c1-300; done
