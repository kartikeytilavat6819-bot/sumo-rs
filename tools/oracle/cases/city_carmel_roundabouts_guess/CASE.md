# Case: city_carmel_roundabouts_guess (Carmel, Indiana city centre, multi-lane roundabouts, US default import), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=-86.1290,39.9750,-86.1160,39.9815` (about 1.0 km x 720 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.63 MB, 1035 ways: 3 `junction=roundabout` ways,
20+ two-lane ways, 221 `highway=crossing`, 52 cycleways.

Stresses netconvert's roundabout import with `--roundabouts.guess` on an American multi-lane roundabout cluster
(lane-count guessing for roundabout rings that are not turbo layouts). Right-hand traffic.

`net.args` = `--roundabouts.guess`. Reference net: 1326 junctions, 5620 edges, 5634 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection --roundabouts.guess -o ref_simple.net.xml`,
`make_sim_refs.sh city_carmel_roundabouts_guess` (sim.args = `--end 600`).
