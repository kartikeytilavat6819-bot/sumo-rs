#!/bin/bash
# C++ reference for the sigma_seed case (WSL, C++ sumo 1.28.0). Same flags as run_all.sh: tripinfo, fcd, statistics,
# --no-step-log, plus sim.args (paths are repo-root relative, so run from the repo root).
# Usage: sim_ref.sh            -> ref.*        from sim.args      (--seed 42)
#        sim_ref.sh seed7      -> ref_seed7.*  from sim_seed7.args (--seed 7)
cd "$(dirname "$0")/../../../.." || exit 1
C=tools/oracle/cases/sigma_seed
V=${1:+_$1}
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n $C/ref_simple.net.xml -r $C/routes.rou \
    --tripinfo-output $C/ref$V.tripinfo.xml --fcd-output $C/ref$V.fcd.xml \
    --statistic-output $C/ref$V.stat.xml --no-step-log true $(cat $C/sim${V:-}.args) > $C/ref$V.log 2>&1
