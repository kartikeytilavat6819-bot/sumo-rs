#!/bin/bash
# C++ netconvert reference for the london sidewalks/bike/turn-lanes variant (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --osm-files ../city_london_lh/input.osm --simple-projection --lefthand \
    --osm.sidewalks --osm.bike-access --osm.turn-lanes \
    -o ref_simple.net.xml
