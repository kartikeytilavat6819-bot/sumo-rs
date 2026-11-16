#!/bin/bash
# Temporary agent sweep: netconvert options on OSM nets, ours vs pristine (removed before merge).
set -u
RS=/tmp/target/debug/netconvert
CP=$SUMO_PRISTINE_BIN/netconvert
C=tools/oracle/cases
W=/tmp/nc
mkdir -p $W
OPTS=(
  "base|"
  "geomRemove|--geometry.remove true"
  "ramps|--ramps.guess true"
  "roundabouts|--roundabouts.guess true"
  "junctionsJoin|--junctions.join true"
  "tlsGuess|--tls.guess-signals true"
  "tlsJoin|--tls.join true"
  "tlsDiscard|--tls.discard-simple true"
  "edgesJoin|--edges.join true"
  "removeIsolated|--remove-edges.isolated true"
  "keepVclass|--keep-edges.by-vclass passenger"
  "removeFootway|--remove-edges.by-type highway.footway,highway.cycleway"
  "sidewalks|--sidewalks.guess true"
  "crossings|--crossings.guess true"
  "turnLanes|--osm.turn-lanes true"
  "laneAccess|--osm.lane-access true"
  "elevation|--osm.elevation true"
  "noInternal|--no-internal-links true"
  "lanewidth|--default.lanewidth 3.2"
  "streetNames|--output.street-names true"
  "originalNames|--output.original-names true"
  "bundle|--geometry.remove true --ramps.guess true --junctions.join true --tls.guess-signals true --tls.discard-simple true --tls.join true --tls.default-type actuated"
)
for net in city_munich real city_plano; do
  for spec in "${OPTS[@]}"; do
    name=${spec%%|*}; args=${spec#*|}
    d=$W/$net/$name; mkdir -p $d
    SUMO_HOME=$SUMO_HOME $CP --simple-projection true $args --osm-files $C/$net/input.osm -o $d/cpp.net.xml > $d/cpp.log 2>&1; crc=$?
    $RS --simple-projection true $args --osm-files $C/$net/input.osm -o $d/rs.net.xml > $d/rs.log 2>&1; rrc=$?
    if grep -q panicked $d/rs.log; then v=PANIC
    elif [ $rrc -ne $crc ]; then v="FAIL exit rs=$rrc cpp=$crc"
    elif [ -s $d/rs.net.xml ] && [ -s $d/cpp.net.xml ]; then
      if python3 tools/oracle/compare_net.py $d/cpp.net.xml $d/rs.net.xml > $d/net.log 2>&1; then v=PASS
      else v="FAIL $(head -1 $d/net.log)"; fi
    else v="FAIL-missing-net"; fi
    echo "$net $name $v"
  done
done
