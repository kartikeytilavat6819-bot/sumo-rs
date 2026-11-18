#!/bin/bash
ls -l $SUMO_PRISTINE_BIN/ | grep -E "od2trips|marouter"
for t in od2trips marouter; do
  for c in $(ls -d tools/oracle/cases/${t}_* | xargs -n1 basename); do
    d=/tmp/${t}$(basename $CARGO_TARGET_DIR)/$c
    for s in ref cand; do echo "== $c $s"; head -c 1500 $d/$s.log; done
    diff -r $d/ref $d/cand | head -20
  done
done
