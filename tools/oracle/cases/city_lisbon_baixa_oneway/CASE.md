# Case: city_lisbon_baixa_oneway (Lisbon Baixa, dense one-way grid, no turnarounds), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-9.1400,38.7075,-9.1365,38.7115` (about 340 m x 440 m, Baixa Pombalina),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.7 MB. Right-hand traffic. Many `oneway=yes`
streets in a tight grid, with dead ends.

`net.args` = `--no-turnarounds`. Turnaround connections are not built at any junction, so vehicles cannot U-turn at
dead ends or at traffic lights. This changes which connections exist on every one-way dead end.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
1208 edges, 25 traffic-light logics.

Routes: 300 cars, one file (randomTrips, seed 1, period 2 s, 600 s):
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 --prefix car -r cars.rou.xml
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --no-turnarounds -o ref_simple.net.xml
make_sim_refs.sh city_lisbon_baixa_oneway                                  # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
