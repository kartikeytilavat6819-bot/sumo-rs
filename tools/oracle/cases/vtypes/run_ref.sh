#!/bin/bash
# Reference run of the C++ SUMO oracle (WSL) on the vtypes case.
cd "$(dirname "$0")"
export SUMO_HOME=$HOME/sumo-src
$HOME/sumo-src/bin/sumo --net-file ../junction/ref_simple.net.xml --route-files routes.rou \
  --additional-files vtypes.add.xml --end 120 --tripinfo-output ref.tripinfo.xml --no-step-log \
  > ref.stdout.txt 2> ref.stderr.txt
echo "ref_exit=$?"
