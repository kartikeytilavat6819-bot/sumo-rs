#!/bin/bash
# C++ netconvert reference for the keepclear net (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
# Variant net for the measurement only: same input with keepClear="true" -> ref_clear.net.xml
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files keepclear.nod.xml --edge-files keepclear.edg.xml --connection-files keepclear.con.xml \
    -o ref_simple.net.xml
