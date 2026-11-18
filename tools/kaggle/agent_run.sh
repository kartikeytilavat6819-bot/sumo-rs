#!/bin/bash
ls -l $SUMO_PRISTINE_BIN/jtrrouter
for c in $(ls tools/oracle/cases | grep ^jtrrouter_); do
  d=/tmp/jtr*/$c; for s in /tmp/jtr[0-7]/$c; do [ -d $s ] || continue
    for f in $s/ref.log $s/cand.log; do echo "== $f"; head -5 $f | cut -c1-200; done
    for f in $s/*.diff; do echo "== $f"; head -12 $f | cut -c1-200; done; done
done
