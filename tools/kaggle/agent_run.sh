#!/bin/bash
# agent z34 (temporary, removed before merge): run every netopts_new row on the gate VM and summarise.
export NETOPTS2_OUT=/tmp/netopts2_z34
bash tools/oracle/run_netopts_new.sh > /tmp/netopts2_z34.log 2>&1
T=$(wc -l < $NETOPTS2_OUT/summary.tsv); P=$(grep -c $'\tPASS\t' $NETOPTS2_OUT/summary.tsv)
NP=$(awk -F'\t' '$2!="PASS"{printf "%s(%s) ", $1, $2}' $NETOPTS2_OUT/summary.tsv)
echo "netopts_new: PASS=$P of $T; not passing: ${NP:-none}"
awk -F'\t' '$2!="PASS"{print $1" "$2" "substr($3,1,300)}' $NETOPTS2_OUT/summary.tsv
