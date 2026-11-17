# Case: city_zurich_tram (central Zurich, tram tracks and tram vehicles), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=8.5380,47.3680,8.5440,47.3715` (about 420 m x 390 m, central Zurich),
retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-cy02`. `input.osm` is 2.1 MB and has 76
`railway=tram` ways. Right-hand traffic, `net.args` is empty.

Structure: 30 `traffic_signals` nodes in the OSM input, 9 traffic-light logics in the reference net, 693 junctions,
1653 edges, 1685 lanes. The OSM bbox has no bus stops, so the buses of the mixed-class idea are not in this case: the
second vehicle class is trams.

Routes: passenger trips plus tram trips, merged into one `routes.rou` (the same merge the Mumbai case uses):
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2  --seed 1 -r routes_car.rou
randomTrips.py -n ref_simple.net.xml -e 600 -p 10 --seed 1 --vehicle-class tram --edge-permission tram --prefix tram -r routes_tram.rou
# merged: 300 passenger vehicles + 60 tram vehicles, one vType "tram_tram" (vClass="tram")
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_zurich_tram                                           # sim.args = --end 600
```
