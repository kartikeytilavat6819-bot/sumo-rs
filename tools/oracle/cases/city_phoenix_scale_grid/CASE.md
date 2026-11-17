# Case: city_phoenix_scale_grid (Phoenix-area suburban grid with highway and actuated signals, 3273 vehicles), net + simulation

Scale case (cy07). Source: OSM via Overpass `/api/map` (overpass.openstreetmap.fr),
`bbox=-112.0100,33.4350,-111.9900,33.4550` (about 1.8 km x 2.2 km, Phoenix/Tempe border, with two
`highway=motorway` ways crossing the box), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 5.3 MB. Right-hand traffic. Net area about 4 km^2 (bbox, not the net hull).

`net.args` = `--tls.default-type actuated`, passed to the reference netconvert and to our netconvert in the net stage.
Every traffic light without its own type gets the actuated controller (the US signal regime).

Structure of the reference net: 16065 edges (highway.motorway x4 in the edge types), 55 traffic-light logics, all `actuated`.

Routes: 3273 vehicles over 1800 s (`randomTrips.py -p 0.55 --fringe-factor 3`, seed 1), one file `routes.rou`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.default-type actuated -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 1800 -p 0.55 --fringe-factor 3 --seed 1 --prefix veh -r routes.rou.xml
sumo -n ref_simple.net.xml -r routes.rou --end 1800 --tripinfo-output ref.tripinfo.xml --statistic-output ref.stat.xml
```
`sim.args` = `--end 1800`. Default `--time-to-teleport` (300 s).
Reference: 925 finished trips; teleports total 331 (jam 219, yield 96).
FCD is not committed (hundreds of MB), so only tripinfo and statistics are compared.
`check_ref_complete.py --end 1800` reports COMPLETE.
