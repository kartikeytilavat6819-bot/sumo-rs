#!/bin/bash
# Regenerates ref_simple.net.xml with the C++ netconvert (flags = net.args) for the persons case.
cd "$(dirname "$0")" || exit 1
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/netconvert --osm-files input.osm --simple-projection $(cat net.args) -o ref_simple.net.xml 2>&1 | tail -5
ls -la ref_simple.net.xml
grep -c 'allow="pedestrian"' ref_simple.net.xml
