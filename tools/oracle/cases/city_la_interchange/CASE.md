# Case: city_la_interchange (Los Angeles four-level interchange), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=-118.252,34.059,-118.243,34.066`, retrieved 2026-11-16.
`input.osm` is 4.9 MB (over the 3 MB soft budget). Contains 68 `motorway_link` and 31 `motorway` mentions (the interchange ramps).

Reference (C++ netconvert and sumo 1.28.0, WSL):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml   # 3687 lanes
randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou       # 150 vehicles
make_sim_refs.sh city_la_interchange   # sim.args = --end 600
```
Reference run: 143 tripinfo entries, FCD to t=600.
