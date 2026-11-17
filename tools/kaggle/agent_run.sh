#!/bin/bash
export STATE_OUT=/tmp/sl06state
bash tools/oracle/run_state.sh rerouter_parking busride person_public_car taxi > /tmp/run_state_out.txt 2>&1
grep -E 'variants=' /tmp/run_state_out.txt
for f in /tmp/sl06state/*/rerouter_parking/sim.state.txt /tmp/sl06state/*/busride/sim.state.txt /tmp/sl06state/*/person_public_car/sim.state.txt; do echo "== $f"; grep -E '^[-+][^-+]' "$f" | head -20 | cut -c1-400; done
