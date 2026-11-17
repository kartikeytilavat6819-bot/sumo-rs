#!/bin/bash
# Per-model reference run of the C++ SUMO oracle (WSL) on the junction net.
# Each model has its own typed route file (cf/<model>.rou.xml: routes.rou with every vehicle on that vType).
# Usage: run_cf_models.sh  -> writes cf/<model>.ref.* and prints "<model>: exit=<n> durations=<list>".
cd "$(dirname "$0")"
export SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo
for m in idm eidm acc cacc cc wiedemann w99 kerner nasch pwag2009 smartsk rail daniel1; do
  ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/sumo --net-file ../junction/ref_simple.net.xml --route-files cf/$m.rou.xml \
    --additional-files cf/$m.add.xml --tripinfo-output cf/$m.ref.tripinfo.xml --no-step-log \
    > cf/$m.ref.stdout.txt 2> cf/$m.ref.stderr.txt
  code=$?
  durs=$(sed -n 's/.*duration="\([0-9.]*\)".*/\1/p' cf/$m.ref.tripinfo.xml | paste -sd, -)
  echo "$m: exit=$code durations=[$durs]"
done
