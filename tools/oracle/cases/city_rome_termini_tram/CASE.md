# Case: city_rome_termini_tram (Rome, Termini station area with tram and bus routes), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=12.4960,41.8980,12.5140,41.9060` (about 1.4 km x 0.9 km,
Termini, Repubblica and Cinquecento), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 8.3 MB. Right-hand traffic. Default netconvert options, no `net.args`.

Structure of the reference net: 6701 edges, 2761 junctions, 120 `tlLogic`, 7528 lanes. The extract has tram and bus
PT lines, and netconvert drops some stops it cannot place on the network (warnings in the reference log).
Pedestrian trips and tram vehicles are not generated: the case covers cars at signals and PT-tagged streets.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_rome_termini_tram      # sim.args = --end 1200
```
Reference run: 350 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
