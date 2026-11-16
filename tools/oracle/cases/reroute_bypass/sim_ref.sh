#!/bin/bash
# C++ reference for the bypass case (WSL, C++ sumo 1.28.0). Same flags as run_all.sh: tripinfo, fcd, statistics,
# --no-step-log, plus sim.args (paths are repo-root relative, so run from the repo root).
# Usage: wsl -e bash /mnt/c/.../agent-gd01/tools/oracle/cases/reroute_bypass/sim_ref.sh
cd "$(dirname "$0")/../../../.." || exit 1
C=tools/oracle/cases/reroute_bypass
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n $C/ref_simple.net.xml -r $C/routes.rou \
    --tripinfo-output $C/ref.tripinfo.xml --fcd-output $C/ref.fcd.xml \
    --statistic-output $C/ref.stat.xml --no-step-log true $(cat $C/sim.args)
