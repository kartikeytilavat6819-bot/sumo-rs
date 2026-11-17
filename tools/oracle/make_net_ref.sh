#!/bin/bash
# Build the C++ reference net for a case directory given as $1 (default: real).
set -u
C=${1:-real}
D=/mnt/c/Users/karti/sumo-rs/tools/oracle/cases/$C
wsl -e bash -lc "SUMO_HOME=\$HOME/sumo-pristine/lib/python3.12/site-packages/sumo \${SUMO_ORACLE_BIN:-\$HOME/sumo-pristine/bin}/netconvert --osm-files $D/input.osm --simple-projection -o $D/ref_simple.net.xml" 2>&1 | tail -5
ls -la /c/Users/karti/sumo-rs/tools/oracle/cases/$C/ref_simple.net.xml 2>&1
