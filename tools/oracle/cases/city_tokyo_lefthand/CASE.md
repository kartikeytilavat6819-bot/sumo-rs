# Case: city_tokyo_lefthand (Shibuya, left-hand traffic), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=139.697,35.656,139.704,35.661`, retrieved 2026-11-16.
`input.osm` is 4.5 MB (over the 3 MB soft budget). Contains trunk and motorway links (the expected arterial mix).

Left-hand traffic: `net.args` = `--lefthand`, which the net stage of `run_all.sh` passes too.

Reference (C++ netconvert and sumo 1.28.0, WSL):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml   # 3347 lanes
randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou                 # 150 vehicles
make_sim_refs.sh city_tokyo_lefthand   # sim.args = --end 600
```
Reference run: 150 tripinfo entries, FCD to t=600.

Note: `net.args` must have LF line endings. A CRLF file passes `--lefthand\r` to netconvert, which then fails.
