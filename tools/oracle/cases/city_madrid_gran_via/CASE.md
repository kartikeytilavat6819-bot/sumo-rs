# Case: city_madrid_gran_via (Madrid Gran Via and Callao, dense signals), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-3.7150,40.4170,-3.6990,40.4240` (about 1.3 km x 0.8 km,
Gran Via between Callao and Plaza de Espana), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 13 MB. Right-hand traffic. Default netconvert options, no `net.args`.

Structure of the reference net: 4138 edges, 1795 junctions, 141 `tlLogic` (traffic-light programs), 4814 lanes.
Pedestrian trips are not generated here: the case covers vehicles at signals only.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_madrid_gran_via        # sim.args = --end 1200
```
Reference run: 324 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
