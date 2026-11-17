# Case: city_paris_etoile (Place Charles de Gaulle / Etoile, multi-lane roundabout), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=2.2915,48.8715,2.2990,48.8760` (about 640 m x 500 m, 8th arrondissement),
retrieved 2026-11-17 from WSL with a User-Agent header (Overpass returns 406 without one). `input.osm` is 4.0 MB,
1197 ways, 10 `junction=circular` ways (the Etoile ring and its segments), 749 `traffic_signals` nodes.
Right-hand traffic, no `net.args`.

Structure of the reference net: one `<roundabout>` element (the Etoile ring guessed by netconvert, 12 ring nodes),
251 traffic-light logics elsewhere in the block, 1303 junctions, 3255 edges, 4377 lanes.
The regime tested is the large multi-lane ring with many entries and exits; the surrounding signals are a side effect.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_paris_etoile                                          # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
