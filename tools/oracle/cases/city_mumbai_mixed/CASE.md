# Case: city_mumbai_mixed (central Mumbai, mixed passenger and bus traffic), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=72.8410,19.0170,72.8470,19.0210` (about 620 m x 440 m, central Mumbai),
retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-cy02`. `input.osm` is 1.5 MB. Right-hand traffic,
`net.args` is empty.

Structure: small irregular street network, 24 `traffic_signals` nodes in the OSM input, 10 traffic-light logics in the
reference net, 351 junctions, 1060 edges, 1147 lanes. Only 2 `lanes=` tags in the input, so lane counts come from the
netconvert defaults.

Routes: passenger trips plus bus trips, merged into one `routes.rou`:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes_car.rou
randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --vehicle-class bus --edge-permission bus --prefix bus -r routes_bus.rou
# merged: 300 passenger vehicles + 120 bus vehicles, one vType "bus_bus" (vClass="bus")
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_mumbai_mixed                                          # sim.args = --end 600
```
