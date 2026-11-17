# Case: city_rotterdam_port (Waalhaven/Botlek industrial port area, trucks only), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=4.3080,51.8820,4.3260,51.8930` (about 1.1 km x 1.2 km, Waalhaven port
industry south of Rotterdam), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.2 MB.
Right-hand traffic, no `net.args`.

Structure: 264 junctions, 2033 edges, 2033 lanes, 108 priority junctions, 24 dead ends, no traffic lights (the port
gates are not mapped as signals in this block). Industrial service roads, and a canal and river that netconvert
discards as unusable.

Vehicle regime: trucks only. The routes are generated with the truck edge permission, so every trip starts and ends on
an edge that allows `truck`:
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou --vehicle-class truck --vclass truck --prefix truck
```
The routes file has about 300 `vehicle` entries (`randomTrips` with `-p 2`, `-e 600`) with `vType id="truck_truck" vClass="truck"`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_rotterdam_port                                        # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
