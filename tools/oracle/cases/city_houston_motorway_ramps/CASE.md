# Case: city_houston_motorway_ramps (Houston freeway interchange, US default import), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=-95.3780,29.7600,-95.3718,29.7640` (about 470 m x 440 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.96 MB, 354 ways: 25 `highway=motorway_link`, 6 `highway=motorway_junction`,
19 `turn:lanes`, 3 `traffic_signals:direction=forward`, 38 `highway=crossing`.

Stresses netconvert's motorway-ramp handling: ramp weaving between freeway lanes, `turn:lanes` with
`merge_to_right|none`, `slight_right` and `left;through` combinations on freeway-to-frontage transitions, and the
US default import (right-hand traffic, no `--lefthand`).

`net.args`: none (netconvert defaults). Reference net: 462 junctions, 1718 edges, 1930 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml`, `make_sim_refs.sh city_houston_motorway_ramps`
(sim.args = `--end 600`).
