#!/bin/bash
# C++ netconvert reference for the train_join net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files train_join.nod.xml --edge-files train_join.edg.xml \
    -o ref_simple.net.xml
