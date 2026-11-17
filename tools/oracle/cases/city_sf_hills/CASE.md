# Case: city_sf_hills (steep streets above Russian Hill / Lombard Street, elevation on), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-122.4150,37.8000,-122.4090,37.8040` (about 500 m x 440 m), retrieved
2026-11-17 from WSL with a User-Agent header. Only 7 OSM nodes in this block carry an `ele` tag, and they are not on
the streets, so the real OSM data gives no elevation to netconvert. The `input.osm` has been changed: every `<node>`
of the file now has a synthetic `<tag k="ele" v="..."/>` from `40 + 35 sin(2400 (lon+122.4150)) + 25 cos(2600 (lat-37.8))`
(metres, range about -18 to 100 m, grades around 10-20 %). The 4 original `ele` nodes are kept. This makes the
`--osm.elevation` path carry real height values; the reference is generated from this same input, so it is a valid
oracle, but it is not measured SF terrain.

`net.args` = `--osm.elevation`, passed to the reference netconvert and to our netconvert in the net stage.
Check: the reference net has 13006 shape points with z, from -18.5 to 99.8 m.

Structure: 712 junctions, 2161 edges, 2225 lanes, 53 traffic-light logics, 286 dead ends, 203 priority junctions.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.elevation -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_sf_hills                                              # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
