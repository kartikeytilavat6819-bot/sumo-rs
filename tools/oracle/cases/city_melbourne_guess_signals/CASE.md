# Case: city_melbourne_guess_signals (Melbourne CBD, signals guessed from OSM nodes), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=144.9620,-37.8130,144.9680,-37.8090` (about 530 m x 440 m, Melbourne CBD),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.9 MB. Left-hand traffic, tram tracks on the streets.

`net.args` = `--tls.guess-signals --tls.guess-signals.dist 25 --lefthand`. Clusters of OSM traffic-signal nodes around one
intersection (within 25 m) are read as signal positions for one larger traffic light.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
2460 edges, 98 traffic-light logics, 0 `rail_crossing` junctions.

Routes: 300 cars, one file (randomTrips, seed 1, period 2 s, 600 s):
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 --prefix car -r cars.rou.xml
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.guess-signals --tls.guess-signals.dist 25 --lefthand -o ref_simple.net.xml
make_sim_refs.sh city_melbourne_guess_signals                              # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
