#!/bin/bash
# C++ netconvert reference for the sublane_bikes net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files sublane_bikes.nod.xml --edge-files sublane_bikes.edg.xml \
    -o ref_simple.net.xml
