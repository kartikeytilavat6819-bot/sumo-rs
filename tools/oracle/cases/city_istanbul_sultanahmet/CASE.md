# Case: city_istanbul_sultanahmet (Istanbul old city, narrow streets and priority junctions), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=28.9690,41.0050,28.9800,41.0110` (about 0.9 km x 0.7 km,
Sultanahmet, Hagia Sophia, Divanyolu and the Grand Bazaar edge), retrieved 2026-11-17 from WSL with a User-Agent
header. `input.osm` is 3.2 MB. Right-hand traffic. Default netconvert options, no `net.args`. This is a different
area from city_istanbul_bosphorus_layers (no bridges or layers here).

Structure of the reference net: 2090 edges, 793 junctions, 88 `tlLogic`, 2090 lanes. Many streets are one lane
and two-way with priority junctions; the signals are at the main avenues.
Pedestrian trips and the tram are not generated: the case covers vehicles on narrow streets.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_istanbul_sultanahmet   # sim.args = --end 1200
```
Reference run: 494 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
