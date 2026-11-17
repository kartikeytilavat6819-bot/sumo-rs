# Case: city_saopaulo_grid (central São Paulo, dense street grid), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-46.6600,-23.5650,-46.6530,-23.5600` (about 620 m x 550 m, central
São Paulo), retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-cy02`. `input.osm` is 3.0 MB. Right-hand
traffic, `net.args` is empty.

Structure: dense orthogonal grid with 118 `traffic_signals` nodes in the OSM input, 53 traffic-light logics in the
reference net, 896 ways, 554 junctions, 1303 edges, 1991 lanes, 69 one-way ways and 18 bus-route tags.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_saopaulo_grid                                         # sim.args = --end 600
```
