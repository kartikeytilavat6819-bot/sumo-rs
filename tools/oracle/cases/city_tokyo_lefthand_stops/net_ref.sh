#!/bin/bash
# C++ netconvert reference for the tokyo stop-length variant (WSL, C++ sumo 1.28.0). Writes ref_simple.net.xml.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-pristine/lib/python3.12/site-packages/sumo ${SUMO_ORACLE_BIN:-$HOME/sumo-pristine/bin}/netconvert \
    --osm-files ../city_tokyo_lefthand/input.osm --simple-projection --lefthand \
    --osm.stop-output.length 12 --osm.stop-output.length.bus 9 \
    -o ref_simple.net.xml
