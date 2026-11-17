#!/bin/bash
bash tools/oracle/run_state.sh > /tmp/run_state_out.txt 2>&1
grep -E 'variants=|STATE DONE' /tmp/run_state_out.txt
grep -E '^(taxi|taxi_closest|rerouter_parking|busride|person_public_car|containers|peds_crossing|persons)/' /tmp/run_state_out.txt
grep -E 'state=(FAIL|NOSTATE)|load=(FAIL|NOLOAD)' /tmp/run_state_out.txt | head -60
