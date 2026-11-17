# Case: city_stevenage_lefthand_roundabouts (Stevenage town centre, UK new town, left-hand traffic with roundabouts), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-0.2060,51.9010,-0.1900,51.9090` (about 1.2 km x 0.9 km),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 3.5 MB.

`net.args` = `--lefthand`. Left-hand traffic and roundabouts (circulation is clockwise in the UK sense).
Reference net: 5350 edges (grep `<edge `).

Routes: 150 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 4 --seed 1 --prefix car -r routes.rou`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
make_sim_refs.sh city_stevenage_lefthand_roundabouts        # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
