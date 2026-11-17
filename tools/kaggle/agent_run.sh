#!/bin/bash
# ss02 temporary runner (removed in the final commit): netopts (188) and the duarouter oracle
export NETOPTS_OUT=/tmp/netopts RUST_NETCONVERT=/tmp/target/debug/netconvert CPP_NETCONVERT=$SUMO_PRISTINE_BIN/netconvert CPP_SUMO_HOME=$SUMO_HOME
bash tools/oracle/run_netopts.sh > /tmp/netopts.log 2>&1
for net in real city_plano city_munich city_berlin_priority city_singapore_lefthand reimport; do
  s=/tmp/netopts/$net/summary.tsv
  echo "netopts $net PASS=$(grep -c $'\tPASS\t' $s 2>/dev/null) FAIL=$(grep -c $'\tFAIL\t' $s 2>/dev/null) PANIC=$(grep -c $'\tPANIC\t' $s 2>/dev/null)"
done
grep -E '^(FAIL|PANIC)' /tmp/netopts.log | head -n 20
bash tools/oracle/run_duarouter.sh 2>&1 | tail -n 3
