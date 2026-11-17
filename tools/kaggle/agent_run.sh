#!/bin/bash
export STATE_OUT=/tmp/sl06state
rm -rf /tmp/sl06state
bash tools/oracle/run_state.sh rerouter_parking > /tmp/run_state_out.txt 2>&1
grep -E 'variants=' /tmp/run_state_out.txt
for f in $(find /tmp/sl06state -name 'sim.save_ours.log'); do echo "== $f"; grep -c SL06TRACE "$f"; grep -A60 -m1 SL06TRACE "$f" | grep -E '^ +[0-9]+: ' | grep -v -E 'std::|core::|alloc::' | head -30 | cut -c1-200; done
