# Case: city_paris_chapelle_ramps (Porte de la Chapelle, Périphérique on/off ramps), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=2.354,48.896,2.362,48.902`, retrieved 2026-11-16.
`input.osm` is 6.1 MB (over the 3 MB soft budget). Right-hand traffic, `net.args` is empty.

Structure: motorway ramps (22 `motorway_link` and 41 `motorway` mentions) feeding a signalised urban arterial. No
roundabout is required by this case. Reference net: 3530 lanes.

Reference (C++ netconvert and sumo 1.28.0, WSL `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou   # 150 vehicles, 1 no-route warning
make_sim_refs.sh city_paris_chapelle_ramps   # sim.args = --end 600, completeness checked
```
Reference run: 150 tripinfo entries, FCD to t=600, stat end=600.

Compare: see `city_plano/CASE.md`.
