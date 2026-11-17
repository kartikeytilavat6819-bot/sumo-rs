# Case: city_paris_dense (Paris Opéra, dense signals), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=2.328,48.868,2.336,48.874`, retrieved 2026-11-16.
`input.osm` is 4.6 MB (over the 3 MB soft budget, kept because the box is the agreed one). Right-hand traffic, `net.args` is empty.

Structure: dense boulevard grid. The OSM input has 692 `traffic_signals` mentions; the reference net has 3912 lanes.

Reference (C++ netconvert and sumo 1.28.0, WSL `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou   # 150 vehicles, 1 no-route warning
make_sim_refs.sh city_paris_dense   # sim.args = --end 600, completeness checked
```
Reference run: 145 tripinfo entries, FCD to t=600, stat end=600.

Compare: see `city_plano/CASE.md`.
