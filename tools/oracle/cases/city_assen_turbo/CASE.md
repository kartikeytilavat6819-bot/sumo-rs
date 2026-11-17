# Case: city_assen_turbo (Dutch roundabout cluster near Assen, --roundabouts.guess), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=6.5570,52.9965,6.5660,53.0000` (about 600 m x 390 m, Assen),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.9 MB, 1072 ways, 6 `junction=roundabout` tags.
The bbox was picked from a 10 MB probe (`6.5560,52.9920,6.5700,53.0010`) as the densest of its two roundabout clusters.
Right-hand traffic.

`net.args` = `--roundabouts.guess`, which guesses roundabout structures from the OSM ways (turbo roundabouts are
multi-lane rings with lane-specific connections).

Structure of the reference net: 987 non-internal edges, 2485 lanes, 647 junctions, 8 traffic-light logics, 3 `<roundabout>` elements.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --roundabouts.guess -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_assen_turbo                                           # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
