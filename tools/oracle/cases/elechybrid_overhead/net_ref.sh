#!/bin/bash
# C++ netconvert reference for the elechybrid_overhead net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo $HOME/sumo-pristine/bin/netconvert \
    --node-files elecov.nod.xml --edge-files elecov.edg.xml \
    -o ref_simple.net.xml
