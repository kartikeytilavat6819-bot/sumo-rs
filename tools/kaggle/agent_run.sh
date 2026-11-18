#!/bin/bash
for c in accept_all turn_ratios; do s=$(ls -d /tmp/jtr[0-7]/jtrrouter_$c | head -1)
  echo "=== $c"; for x in ref cand; do echo "--- $x"; grep -v "generated on\|^<?xml\|^$" $s/$x/routes.rou.xml | cut -c1-170; done
done
