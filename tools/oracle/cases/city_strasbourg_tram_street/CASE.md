# Case: city_strasbourg_tram_street (Strasbourg, tram Ligne D in the street, EU default import), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=7.7470,48.5818,7.7505,48.5833` (about 270 m x 170 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.2 MB, 365 ways: 4 `railway=tram` ways and tram
crossings, 4 `highway=crossing`, 2 `oneway:bicycle=no` (contraflow bike access on one-way streets).

Stresses netconvert's OSM import for a tram route sharing the carriageway (`railway=tram` on a road way) and
one-way streets with contraflow bicycle access. Right-hand traffic, default EU import.

`net.args`: none. Reference net: 84 junctions, 204 edges, 208 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml`, `make_sim_refs.sh city_strasbourg_tram_street`
(sim.args = `--end 600`).
