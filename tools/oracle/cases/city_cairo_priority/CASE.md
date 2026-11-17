# Case: city_cairo_priority (central Cairo, dense unsignalized priority junctions), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=31.2300,30.0400,31.2380,30.0460` (about 770 m x 670 m, downtown Cairo near Tahrir),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 1.0 MB, 418 ways, 57 `traffic_signals` nodes, 2 `junction=roundabout`.
Right-hand traffic, dense grid of short streets with priority (yield/stop) junctions.

`net.args` = `--tls.discard-simple`. Traffic lights at simple junctions are discarded, so most of the 57 OSM signal nodes
become unsignalized priority junctions. The reference net keeps 4 traffic-light logics.

Structure of the reference net: 590 non-internal edges, 1912 lanes, 405 junctions, 4 `traffic_light` junctions, 2 `<roundabout>` elements.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.discard-simple -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_cairo_priority                                        # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
