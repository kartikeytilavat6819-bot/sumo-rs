# Case: city_groningen_bike_contraflow (Groningen centre, one-way streets with bicycle contraflow), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=6.5630,53.2170,6.5700,53.2195` (about 500 m x 280 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.99 MB (under the 2 MB budget), 637 ways: 9 `oneway:bicycle=no`
ways on one-way car streets (contraflow cycling), 2 `highway=cycleway`, 2 `highway=crossing`, 159 highway ways.

Stresses netconvert's one-way handling with `oneway:bicycle=no`, where a one-way car edge gets a bicycle-only
opposite lane. Right-hand traffic, default EU import.

`net.args`: none. Reference net: 233 junctions, 604 edges, 604 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml`, `make_sim_refs.sh city_groningen_bike_contraflow`
(sim.args = `--end 600`).
