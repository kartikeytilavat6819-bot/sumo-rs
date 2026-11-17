# Case: city_chicago_loop (Chicago Loop, street grid with elevated rail), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-87.6340,41.8770,-87.6200,41.8860` (about 1.2 km x 1.0 km,
the Chicago Loop: State, Wabash, Lake, Wacker), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 10 MB. Right-hand traffic. Default netconvert options, no `net.args`.

The Loop's elevated CTA tracks (`railway=rail` on `bridge`/`layer` ways) are imported as rail edges and are not
used by the car trips. netconvert discards some other `railway` ways with warnings in the reference log.

Structure of the reference net: 9863 edges, 4711 junctions, 421 `tlLogic` (dense signal grid), 12564 lanes.
Pedestrian trips are not generated here: the case covers vehicles at signals and rail crossings.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_chicago_loop           # sim.args = --end 1200
```
Reference run: 427 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
