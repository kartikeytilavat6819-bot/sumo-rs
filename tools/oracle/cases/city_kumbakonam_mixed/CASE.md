# Case: city_kumbakonam_mixed (small Indian town, mixed cars and motorcycles, no lane markings, --osm.lane-access), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=79.3740,10.9580,79.3850,10.9650` (about 1.0 km x 780 m, Kumbakonam centre),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 0.2 MB, 124 ways, 864 nodes, 100 `highway` tags.
Right-hand traffic. No `traffic_signals` in the input, and no lane markings are tagged, so lanes come from netconvert defaults.

`net.args` = `--osm.lane-access`, which imports lane-specific access restrictions (no effect on untagged lanes).

Structure of the reference net: 452 non-internal edges, 2126 lanes, 498 junctions, 0 traffic-light logics.

Routes: mixed vehicle classes, one file, 500 vehicles. Random O/D routes from randomTrips:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes_car.rou                                            # 300 passenger cars
randomTrips.py -n ref_simple.net.xml -e 600 -p 3 --seed 2 --vehicle-class motorcycle --edge-permission motorcycle --prefix moto -r routes_moto.rou   # 200 motorcycles
# merged into routes.rou: vType "moto_motorcycle" (vClass="motorcycle"), vehicles sorted by depart, routes_car.rou and routes_moto.rou removed
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.lane-access -o ref_simple.net.xml
make_sim_refs.sh city_kumbakonam_mixed                                      # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
