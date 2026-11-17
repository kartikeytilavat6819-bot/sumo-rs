# Case: city_moscow_mkad_ramps (Moscow MKAD ring road segment, on- and off-ramps guessed, high flow), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=37.6100,55.5710,37.6600,55.5790` (about 3.7 km x 0.9 km, south MKAD near
km 28-33; the ring road is `highway=trunk` with ref `МКАД, N-й километр`), retrieved 2026-11-17 from WSL with a
User-Agent header. `input.osm` is 5.3 MB. Right-hand traffic. Dual carriageway ring with ramps to city streets.

`net.args` = `--ramps.guess`. Ramp-guessing adds on-ramps and off-ramps where a fast edge meets a slower one.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
4583 edges, 1071 junctions, 11 traffic-light logics.

Routes: 750 cars (high flow, period 0.8 s), one file:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 0.8 --seed 1 --prefix car -r cars.rou.xml
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --ramps.guess -o ref_simple.net.xml
make_sim_refs.sh city_moscow_mkad_ramps                                    # sim.args = --end 600
```
Reference run: tripinfo and statistics written by the C++ sumo, checked by `check_ref_complete.py` (before the FCD was removed).
The FCD output (21 MB for 750 vehicles) is not committed, so only tripinfo and statistics are compared.
