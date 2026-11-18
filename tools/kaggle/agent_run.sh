#!/bin/bash
for c in turn_ratios turn_defaults discount random; do s=$(ls -d /tmp/jtr[0-7]/jtrrouter_$c | head -1)
  echo "=== $c"; head -24 $s/routes.rou.xml.diff | cut -c1-200; head -3 $s/cand.log $s/ref.log | cut -c1-150
done
