#!/bin/bash
export STATE_OUT=/tmp/sl06state
rm -rf /tmp/sl06state
bash tools/oracle/run_state.sh rerouter_parking > /tmp/run_state_out.txt 2>&1
grep -E 'variants=' /tmp/run_state_out.txt
f=$(find /tmp/sl06state -name 'sim.save_ours.log' | head -1)
grep -m1 SL06TRACE "$f" | grep -o 'fn: "[^"]*"' | grep -v -E 'std::|core::|alloc::|backtrace' | head -25
grep -m1 SL06TRACE "$f" | cut -c1-600
