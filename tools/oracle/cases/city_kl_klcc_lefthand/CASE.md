# Case: city_kl_klcc_lefthand (Kuala Lumpur KLCC, left-hand traffic and bus routes), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=101.7060,3.1520,101.7200,3.1640` (about 1.5 km x 1.3 km,
KLCC, Jalan Ampang and Suria KLCC), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 6.0 MB.
Left-hand traffic (Malaysia).

`net.args` = `--lefthand`, passed to the reference netconvert. Different area and country from
city_singapore_lefthand (central Singapore, also left-hand).

Structure of the reference net: 14590 edges, 4189 junctions, 84 `tlLogic`, 16229 lanes. The extract has hop-on
bus stops and LRT/monorail ways; netconvert discards the unusable ones with warnings.
Pedestrian trips are not generated: the case covers vehicles on left-hand streets with signals.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
make_sim_refs.sh city_kl_klcc_lefthand       # sim.args = --end 1200
```
Reference run: 468 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
