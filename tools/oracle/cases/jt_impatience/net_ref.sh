#!/bin/bash
# C++ netconvert reference for the jt_impatience net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files junction.nod.xml --edge-files junction.edg.xml --connection-files junction.con.xml \
    -o ref_simple.net.xml
