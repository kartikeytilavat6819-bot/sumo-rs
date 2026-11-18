# Case: city_dublin_tram_crossings (Dublin city centre, Luas trams in the street, left-hand traffic), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=-6.2600,53.3490,-6.2568,53.3506` (about 260 m x 180 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.4 MB, 154 ways: 22 `railway=tram` mentions,
13 `railway=tram_level_crossing` and 7 `railway=tram_crossing` nodes/ways (trams crossing carriageways), 1 `traffic_signals:direction=both`.

Stresses netconvert's handling of tram tracks next to road lanes (tram ways are not road edges) and of
tram crossings at signal-controlled junctions under `--lefthand`.

`net.args` = `--lefthand`. Reference net: 104 junctions, 260 edges, 267 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml` (C++ 1.28.0, WSL pristine),
then `make_sim_refs.sh city_dublin_tram_crossings` (sim.args = `--end 600`).
