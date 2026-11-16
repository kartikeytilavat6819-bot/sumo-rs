#!/bin/bash
# C++ reference for the reroute_contention case (WSL, C++ sumo 1.28.0). Same flags as run_all.sh: tripinfo, fcd,
# statistics, --no-step-log, plus sim.args (paths are repo-root relative, so run from the repo root).
# Usage: wsl -e bash /mnt/c/.../agent-fx04-oracle/tools/oracle/cases/reroute_contention/sim_ref.sh [adapt]
#   no argument: ref.* from sim.args;  "adapt": ref_adapt.* from sim_adapt.args
cd "$(dirname "$0")/../../../.." || exit 1
C=tools/oracle/cases/reroute_contention
TAG=ref; ARGS=$C/sim.args
if [ "${1:-}" = adapt ]; then TAG=ref_adapt; ARGS=$C/sim_adapt.args; fi
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/sumo -n $C/ref_simple.net.xml -r $C/routes.rou \
    --tripinfo-output $C/$TAG.tripinfo.xml --fcd-output $C/$TAG.fcd.xml \
    --statistic-output $C/$TAG.stat.xml --no-step-log true $(cat $ARGS)
