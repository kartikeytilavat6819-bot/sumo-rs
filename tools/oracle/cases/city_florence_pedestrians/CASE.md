# Case: city_florence_pedestrians (Florence old town, pedestrian-heavy, sidewalks and crossings guessed, persons walking), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=11.2530,43.7680,11.2600,43.7730` (about 550 m x 550 m, Duomo and centre),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 3.9 MB. Right-hand traffic. Narrow streets,
many `highway=footway`/`pedestrian` ways, few `traffic_signals`.

`net.args` = `--sidewalks.guess --crossings.guess`. Sidewalks are guessed from edge speed; pedestrian crossings are
guessed where sidewalks meet a road at uncontrolled nodes.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
4020 edges, 760 junctions, 2 traffic-light logics. About 1400 edges allow `pedestrian` (guessed sidewalks).

Routes: 100 cars (period 6 s) and 300 persons walking (`--persontrips`, period 2 s), one file, sorted by depart:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 6 --seed 1 --prefix car -r cars.rou.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 2 --prefix person --persontrips -r persons.rou.xml
# merged into routes.rou by depart
```
Each person is `<person>` with a `<walk edges=...>` route. 142 persons finish before the 600 s end.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --sidewalks.guess --crossings.guess -o ref_simple.net.xml
make_sim_refs.sh city_florence_pedestrians                                 # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
