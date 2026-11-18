#!/bin/bash
ls -l $SUMO_PRISTINE_BIN/netgenerate $SUMO_PRISTINE_BIN/polyconvert 2>&1
ls -l $CARGO_TARGET_DIR/debug/netgenerate $CARGO_TARGET_DIR/debug/polyconvert 2>&1
ls $SUMO_HOME/data/typemap | head -30
d=/tmp/z32; rm -rf $d; mkdir -p $d
$SUMO_PRISTINE_BIN/polyconvert --osm-files tools/oracle/cases/real/input.osm -o $d/o.xml 2>&1 | tail -5; ls -l $d
