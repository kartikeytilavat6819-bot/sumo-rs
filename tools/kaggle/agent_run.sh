#!/bin/bash
# agent z25 temporary gate: netconvert option rows (remove before merge)
OUT=/tmp/netopts_z25
export NETOPTS2_OUT=$OUT
bash tools/oracle/run_netopts_new.sh > $OUT.log 2>&1
tail -1 $OUT.log
N=$(wc -l < $OUT/summary.tsv)
P=$(grep -c $'\tPASS\t' $OUT/summary.tsv)
NP=$(awk -F'\t' '$2!="PASS"{printf "%s ", $1}' $OUT/summary.tsv)
echo "netopts_new: PASS=$P of $N; not passing: $NP"
awk -F'\t' '$2!="PASS"{print $1" "$2": "substr($3,1,400)}' $OUT/summary.tsv
