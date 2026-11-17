# Case: city_vienna_scale_downtown (dense Vienna inner city, static signals, 3001 vehicles), net + simulation

Scale case (cy07). Source: OSM via Overpass `/api/map` (overpass.openstreetmap.fr),
`bbox=16.3650,48.2060,16.3780,48.2130` (about 1.0 km x 0.8 km, around St. Stephen's Cathedral, Innere Stadt),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 8.7 MB. Right-hand traffic.
Net area about 0.8 km^2.

`net.args` = `--tls.default-type static` (explicit; traffic lights keep their OSM-given static type).

Structure of the reference net: 5077 edges (1512 internal), 2354 junctions, 25 traffic-light logics.

Routes: 3001 vehicles over 1800 s (`randomTrips.py -p 0.6 --fringe-factor 2`, seed 1), one file `routes.rou`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.default-type static -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 1800 -p 0.6 --fringe-factor 2 --seed 1 --prefix veh -r routes.rou.xml
sumo -n ref_simple.net.xml -r routes.rou --end 1800 --tripinfo-output ref.tripinfo.xml --statistic-output ref.stat.xml
```
`sim.args` = `--end 1800`. Default `--time-to-teleport` (300 s).
Reference: 544 finished trips; teleports total 323 (jam 182, yield 141). Gridlock in the centre is the point of the case.
FCD is not committed (it would be hundreds of MB for 3001 vehicles), so only tripinfo and statistics are compared.
`check_ref_complete.py --end 1800` reports COMPLETE.
