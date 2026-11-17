#!/bin/bash
# C++ netconvert reference for the sl2015 net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files sl2015.nod.xml --edge-files sl2015.edg.xml \
    -o ref_simple.net.xml
