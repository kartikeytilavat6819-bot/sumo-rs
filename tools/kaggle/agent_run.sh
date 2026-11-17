#!/bin/bash
export STATE_OUT=/tmp/sl06state
bash tools/oracle/run_state.sh rerouter_parking busride person_public_car taxi > /tmp/run_state_out.txt 2>&1
head -30 /tmp/run_state_out.txt | cut -c1-300
find /tmp/sl06state -name '*.state.txt' | head -20
for f in $(find /tmp/sl06state -name 'sim.state.txt' | grep -E 'rerouter_parking|busride|person_public_car'); do echo "== $f"; grep -E '^[-+][^-+]' "$f" | head -20 | cut -c1-400; done
