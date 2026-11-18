# Case: city_seattle_first_hill (Seattle Pioneer Square / International District up to First Hill, elevation option), net + simulation

Replaces city_seattle_downtown_hills (parked: the official C++ reads `myConflicts` out of bounds there, see its PENDING).
Same structure, flags and demand; the area is the bbox directly south of it.

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-122.3420,47.5950,-122.3250,47.6040` (about 1.3 km x 1.0 km),
retrieved 2026-11-17 from WSL. `input.osm` is 8.3 MB. Right-hand traffic.

`net.args` = `--osm.elevation`. Like the old case, the extract has only 6 `ele` tags and none on road nodes, so every
junction stays at z=0 (no grades): the option path is exercised, the grades are not (see city_stelvio_pass_grades).

Reference net: 12551 edges, 4213 junctions, 390 `tlLogic`, 14631 lanes.

Routes: 600 cars from `randomTrips.py` (`--validate`, seed 1, period 2 s, 1200 s):
```
randomTrips.py -n ref_simple.net.xml -e 1200 -p 2 --seed 1 --validate -r routes.rou
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.elevation -o ref_simple.net.xml
make_sim_refs.sh city_seattle_first_hill   # NEW_CASE marker; sim.args = --end 1200
```
Reference run: 406 completed trips, FCD and statistics, checked by `check_ref_complete.py`.

UB check: an instrumented official C++ (log whenever `foeConflictIndex >= foeExitLink->myConflicts.size()` in
`ConflictInfo::getFoeLengthBehindCrossing` / `getFoeConflictSize`) reports 0 hits on this case (sw02).
