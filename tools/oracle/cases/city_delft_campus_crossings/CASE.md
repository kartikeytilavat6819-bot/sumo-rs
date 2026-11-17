# Case: city_delft_campus_crossings (TU Delft campus, pedestrians and walking persons), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=4.3660,51.9975,4.3790,52.0030` (about 0.9 km x 0.6 km, Delft
University of Technology campus and surroundings), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 4.0 MB. Right-hand traffic.

`net.args` = `--sidewalks.guess --crossings.guess`. Reference net: 6185 edges. The guessers added no `<crossing>`
or `<walkingArea>` elements in this box, so the crossing logic is exercised only through sidewalks and pedestrian
edges. The case is kept because the persons and sidewalks still stress the pedestrian model.

Routes (merged by `departure` order): 75 cars (`randomTrips.py -p 8 --seed 1`) and 200 persons walking
(`randomTrips.py --persontrips -p 3 --seed 2 --prefix person`), 600 s.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --sidewalks.guess --crossings.guess -o ref_simple.net.xml
make_sim_refs.sh city_delft_campus_crossings        # sim.args = --end 600
```
Extra output: `--personinfo-output personinfo` (`extra_outputs.txt`, compared as `ref.personinfo.xml`).
Reference run: tripinfo, FCD, statistics and personinfo written by the C++ sumo, checked by `check_ref_complete.py`.
