# Case: city_singapore_lefthand (central Singapore, left-hand traffic), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=103.8530,1.2780,103.8600,1.2830` (about 610 m x 560 m, central Singapore),
retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-cy02`. `input.osm` is 2.1 MB. The bbox has no
`highway=motorway` or `trunk` ways, so this case covers left-hand urban traffic only, not an expressway. The name
was kept short for the set; the expressway feature is not tested here.

`net.args` = `--lefthand`, passed to the reference netconvert and to our netconvert in the net stage.

Structure: 105 `traffic_signals` nodes in the OSM input, 42 traffic-light logics in the reference net, 405 junctions,
1421 edges, 2036 lanes, 134 one-way ways.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_singapore_lefthand                                    # sim.args = --end 600
```
