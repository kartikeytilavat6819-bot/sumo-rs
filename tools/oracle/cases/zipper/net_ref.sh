#!/bin/bash
# C++ netconvert reference for the zipper net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/netconvert \
    --node-files zipper.nod.xml --edge-files zipper.edg.xml \
    -o ref_simple.net.xml
