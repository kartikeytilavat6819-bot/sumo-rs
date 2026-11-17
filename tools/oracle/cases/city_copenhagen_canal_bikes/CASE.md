# Case: city_copenhagen_canal_bikes (Christianshavn canals and bike lanes, --osm.bike-access), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=12.5900,55.6730,12.5980,55.6780` (about 640 m x 555 m, Christianshavn),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 1.5 MB, 588 ways, 4 `waterway=canal` tags,
39 `cycleway` tags, 43 `bicycle` tags. Right-hand traffic.

`net.args` = `--osm.bike-access`, which imports bike lanes and fixes their directions. Routes are bicycle trips only, so the
bike-lane network is what gets used.

Structure of the reference net: 520 non-internal edges, 1771 lanes, 394 junctions, 5 traffic-light logics, no roundabout.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.bike-access -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 --vehicle-class bicycle --edge-permission bicycle --prefix bike -r routes.rou   # 300 bicycles
make_sim_refs.sh city_copenhagen_canal_bikes                                # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
