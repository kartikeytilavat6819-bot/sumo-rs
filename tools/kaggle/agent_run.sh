#!/bin/bash
ls -l $SUMO_PRISTINE_BIN/ | grep -E "od2trips|marouter"
cargo build -p od2trips -p marouter --message-format short 2>&1 | grep -E "error|warning: unused" | head -40
for t in od2trips marouter; do
  O=/tmp/agz31_$t; OUT=$O bash tools/oracle/run_odtool.sh $t > /dev/null 2>&1
  for c in $(grep -v $'\tPASS\t' $O/results.tsv | tail -n +2 | cut -f1); do
    echo "######## $c"; cat tools/oracle/cases/$c/args
    for s in ref cand; do echo "== $s"; head -c 1200 $O/$c/$s.log; done
    diff -r $O/$c/ref $O/$c/cand | head -16
  done
done
