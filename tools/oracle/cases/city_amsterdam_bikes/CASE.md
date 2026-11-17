# Case: city_amsterdam_bikes (Leidseplein, cycle lanes), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=4.877,52.362,4.886,52.367`, retrieved 2026-11-16.
`input.osm` is 4.9 MB (over the 3 MB soft budget). The OSM input has 98 cycleway tags and 75 bicycle tags.

Structure: bicycle-heavy centre, 2620 lanes in the reference net.

Reference (C++ netconvert and sumo 1.28.0, WSL):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml   # 2620 lanes
randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 --vehicle-class bicycle --edge-permission bicycle --prefix bike -r routes.rou
make_sim_refs.sh city_amsterdam_bikes   # sim.args = --end 600
```
Reference run: 150 tripinfo entries, FCD to t=600. The generator reported 1 trip with no valid route (warning only).
