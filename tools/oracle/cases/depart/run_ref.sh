#!/bin/bash
# Reference run of the C++ SUMO oracle (WSL) on every depart variant.
cd "$(dirname "$0")"
export SUMO_HOME=$HOME/sumo-src
NET=../junction/ref_simple.net.xml
for r in *.rou; do
  n=${r%.rou}
  mkdir -p ref/$n
  $HOME/sumo-src/bin/sumo -n $NET -r $r --tripinfo-output ref/$n/tripinfo.xml \
    --statistic-output ref/$n/statistic.xml --seed 42 --end 120 --no-step-log \
    > ref/$n/stdout.txt 2> ref/$n/stderr.txt
  echo "$n ref_exit=$?"
done
