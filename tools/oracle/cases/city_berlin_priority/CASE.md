# Case: city_berlin_priority (central Berlin Mitte, priority junctions and traffic lights), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=13.3740,52.5085,13.3800,52.5120` (about 430 m x 380 m, central Berlin),
retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-cy02` (Overpass is blocked or 502 on this machine).
`input.osm` is 5.4 MB, which is over the 3 MB soft budget: the bbox is already the smallest useful block. Right-hand
traffic, `net.args` is empty.

Structure: a mixed centre with 146 `traffic_signals` nodes in the OSM input, 36 traffic-light logics in the reference
net, and many priority (non-signalised) junctions. The OSM input has 1236 ways, 5 tram-rail ways and 21 bus routes.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml   # 948 junctions, 2009 edges, 2279 lanes
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_berlin_priority                                        # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
