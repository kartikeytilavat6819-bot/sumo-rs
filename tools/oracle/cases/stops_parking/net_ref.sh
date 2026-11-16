#!/bin/bash
# C++ netconvert reference for the stops_parking net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/netconvert \
    --node-files stops.nod.xml --edge-files stops.edg.xml \
    -o ref_simple.net.xml
