# Case: city_autobahn_interchange (A5/A6 Autobahnkreuz Walldorf, motorway interchange, high speeds), net + simulation

Source: OSM via Overpass, highway ways only: `way["highway"](49.2790,8.6150,49.2860,8.6290); (._;>;); out meta;`
(a bbox around the Kreuz Walldorf junction nodes). The query is not `/api/map`: a full map of this block is 13.7 MB,
mostly buildings and landuse that the test does not need. `input.osm` is 154 KB, 65 ways, 58 of them motorway or
motorway link. Retrieved 2026-11-17 from WSL with a User-Agent header. Right-hand traffic, no `net.args`.

Structure: 88 junctions, 310 edges, 349 lanes, 43 motorway lanes, 0 traffic lights (the interchange is priority and
ramp merges only). The regime tested is motorway speeds, on/off ramps and merges, and turning radii at interchange
curves (netconvert warns about speed reduction on sharp connections).

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_autobahn_interchange                                  # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
