#!/bin/bash
# C++ netconvert reference for the stop-and-go net shared by the cf_* cases (WSL, C++ sumo 1.28.0).
# Writes ref_simple.net.xml here; each cf_* case dir holds a copy.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --node-files stopgo.nod.xml --edge-files stopgo.edg.xml --tllogic-files stopgo.tll.xml \
    -o ref_simple.net.xml
