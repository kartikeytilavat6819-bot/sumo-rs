#!/bin/bash
# Rebuild the C++ references with --simple-projection (the only projection the Rust port can do)
# then compare our netconvert output against them for every corpus case.
set -u
cd /c/Users/karti/sumo-rs || exit 1
OUT=/c/Users/karti/Temp/oracle
mkdir -p "$OUT"
for c in junction roundabout signal; do
  WSLDIR=/mnt/c/Users/karti/sumo-rs/tools/oracle/cases/$c
  wsl -e bash -lc "SUMO_HOME=\$HOME/sumo-src \$HOME/sumo-src/bin/netconvert --osm-files $WSLDIR/input.osm --simple-projection -o $WSLDIR/ref_simple.net.xml" >"$OUT/ref_$c.log" 2>&1
  echo "== $c reference exit: $?"
done
ls -la tools/oracle/cases/*/ref_simple.net.xml 2>&1 | head -5
