#!/bin/bash
for s in /tmp/jtr[0-7]; do [ -d $s/jtrrouter_turn_ratios ] || continue
for c in $(ls $s | grep ^jtrrouter_); do f=$s/$c/routes.rou.xml.diff; [ -s $f ] || continue
  echo "=== $c"; head -16 $f | cut -c1-200; head -3 $s/$c/cand.log | cut -c1-150
done; break; done
