#!/bin/bash
# C++ reference for the rail case (WSL, C++ sumo 1.28.0). Same flags as run_all.sh: tripinfo, fcd, statistics,
# --no-step-log, plus sim.args (paths are repo-root relative, so run from the repo root). Warnings go to ref.log.
cd "$(dirname "$0")/../../../.." || exit 1
C=tools/oracle/cases/rail
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n $C/ref_simple.net.xml -r $C/routes.rou \
    --tripinfo-output $C/ref.tripinfo.xml --fcd-output $C/ref.fcd.xml \
    --statistic-output $C/ref.stat.xml --no-step-log true $(cat $C/sim.args) > $C/ref.log 2>&1
