# Case: city_budapest_multileg_junctions (Budapest, Deak Ferenc ter area, dense 4-5 leg junctions and trams), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=19.0530,47.4975,19.0565,47.4990` (about 260 m x 170 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.65 MB, 241 ways: 134 highway ways, 8 `railway=tram` ways,
9 `traffic_signals:direction`, 7 `turn:lanes`, 16 `highway=crossing`. Highest leg counts at a node: 5 incident highway ways.

Stresses netconvert's junction shaping where five roads meet close together with tram tracks and signal-direction
tags. Note: no junction here has more than 5 legs (the 6+ leg search found none in the areas fetched), so this is a
5-leg cluster case, not a 7+ leg one. Right-hand traffic, default EU import.

`net.args`: none. Reference net: 223 junctions, 436 edges, 489 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml`, `make_sim_refs.sh city_budapest_multileg_junctions`
(sim.args = `--end 600`).
