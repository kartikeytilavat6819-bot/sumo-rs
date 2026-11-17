# Case: city_portland_actuated (downtown Portland grid, actuated signals), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-122.6810,45.5160,-122.6720,45.5220` (about 800 m x 670 m, downtown Portland,
the Park Blocks grid), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 3.1 MB.

`net.args` = `--tls.default-type actuated`, passed to the reference netconvert and to our netconvert in the net stage.
Every traffic light without its own type gets the actuated controller, so the signal regime is actuated, not static.

Structure: 1318 junctions, 3881 edges, 5045 lanes, 354 `actuated` traffic-light logics in the reference net, 459 dead ends.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.default-type actuated -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_portland_actuated                                     # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
