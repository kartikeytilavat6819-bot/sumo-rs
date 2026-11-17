# Case: city_seattle_downtown_hills (Seattle downtown and Pike/Pine, elevation option), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-122.3420,47.6040,-122.3250,47.6130` (about 1.4 km x 1.0 km,
downtown Seattle up toward Pike/Pine), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 8.9 MB. Right-hand traffic.

`net.args` = `--osm.elevation`, passed to the reference netconvert. Known limit: this OSM extract has no `ele` tags
in the bbox, so every junction stays at z=0 in `ref_simple.net.xml` (no grades). The option path is exercised, but
the hill grades are not. A case with `ele` tags (see city_stelvio_pass_grades) covers real grades.

Structure of the reference net: 12518 edges, 4900 junctions, 551 `tlLogic`, 15420 lanes. Pedestrian trips and the
streetcar are not generated: the case covers vehicles at signals.

Routes: 600 cars from `randomTrips.py`, one file (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.elevation -o ref_simple.net.xml
make_sim_refs.sh city_seattle_downtown_hills # sim.args = --end 1200
```
Reference run: 444 completed trips, FCD and statistics, checked by `check_ref_complete.py`.
