# Case: city_german_level_crossing (German town, level railway crossings with barriers, cars and trains), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=8.6640,49.5560,8.6780,49.5620` (about 1.0 km x 0.7 km, Germany, Baden-Württemberg),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 3.5 MB.
Right-hand traffic. Railway lines with `railway=level_crossing` nodes that carry `crossing:barrier` tags.

`net.args` = `--railway.topology.repair`. Level crossings only become `rail_crossing` junctions when the node has
`crossing:barrier` (not `no`), `crossing.light=yes`, or a traffic light; in this bbox these come from the OSM tags.
Without the barrier tag the crossing stays a plain junction, so the bbox was chosen where the tags exist.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
1893 edges, 373 junctions, 3 `rail_crossing` junctions, 3 traffic-light logics, 109 edges allowing `rail`.

Routes: 300 cars and 20 trains (vehicle class `rail`, vType `train_rail` written by randomTrips), one file:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 --prefix car -r cars.rou.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 30 --seed 2 --prefix train --vehicle-class rail --edge-permission rail -r trains.rou.xml
# merged into routes.rou by vehicle depart (vType train_rail kept from trains.rou.xml)
```
Some train trips have no valid route (randomTrips warnings) and are dropped.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --railway.topology.repair -o ref_simple.net.xml
make_sim_refs.sh city_german_level_crossing                                 # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
