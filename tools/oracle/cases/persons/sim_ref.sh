#!/bin/bash
# Regenerates ref.tripinfo.xml (persons appear as <personinfo>) with the C++ sumo and the exact run_all flags.
cd "$(dirname "$0")/../../../.." || exit 1
d=tools/oracle/cases/persons
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n $d/ref_simple.net.xml -r $d/routes.rou $(cat $d/sim.args) \
    --tripinfo-output $d/ref.tripinfo.xml --no-step-log true
echo "exit $?"
