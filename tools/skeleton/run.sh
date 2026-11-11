#!/bin/bash
# Regenerate crates/sumo_core/src from SUMO's C++ (run inside WSL from tools/skeleton).
# WARNING: overwrites crates/sumo_core/src - only use before agents start filling in bodies.
set -e
cd "$(dirname "$0")"
python3 known_types.py
rm -rf ../../crates/sumo_core/src
python3 gen.py activitygen activitygen/activities activitygen/city dfrouter duarouter jtrrouter libsumo marouter \
  mesosim microsim microsim/actions microsim/cfmodels microsim/devices microsim/engine microsim/lcmodels \
  microsim/logging microsim/output microsim/traffic_lights microsim/transportables microsim/trigger \
  netbuild netgen netimport netimport/vissim netimport/vissim/tempstructs netimport/vissim/typeloader \
  netload netwrite od polyconvert router traci-server utils/emissions utils/handlers utils/router \
  utils/shapes utils/vehicle --out ../../crates/sumo_core/src
