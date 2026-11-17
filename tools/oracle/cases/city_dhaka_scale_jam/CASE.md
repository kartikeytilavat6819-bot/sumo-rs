# Case: city_dhaka_scale_jam (Dhaka Gulshan/Banani mixed grid, left-hand, congested, teleports, 4000 vehicles), net + simulation

Scale case (cy07). Source: OSM via Overpass `/api/map` (overpass-api.de),
`bbox=90.3950,23.7850,90.4200,23.8000` (about 2.5 km x 1.7 km, Gulshan–Banani–Mohakhali), retrieved
2026-11-17 from WSL with a User-Agent header. `input.osm` is 9.9 MB. Left-hand traffic (Bangladesh).
Net area about 4.3 km^2 (bbox).

`net.args` = `--tls.default-type static --lefthand`, passed to the reference netconvert and to our netconvert in the net stage.

Structure of the reference net: 7629 edges, 1843 junctions, 12 traffic-light logics; motorway/trunk/primary/secondary
mix with many residential and track edges.

Routes: 4000 vehicles over 1800 s (`randomTrips.py -p 0.45 --fringe-factor 5`, seed 1), one file `routes.rou`.
High fringe factor pushes trips onto the boundary edges, so the same narrow streets carry most of the traffic.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.default-type static --lefthand -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 1800 -p 0.45 --fringe-factor 5 --seed 1 --prefix veh -r routes.rou.xml
sumo -n ref_simple.net.xml -r routes.rou --end 1800 --tripinfo-output ref.tripinfo.xml --statistic-output ref.stat.xml
```
`sim.args` = `--end 1800`. Default `--time-to-teleport` (300 s), so long jams produce teleports.
Reference: 797 finished trips; teleports total 662 (jam 318, yield 334).
FCD is not committed (hundreds of MB), so only tripinfo and statistics are compared.
`check_ref_complete.py --end 1800` reports COMPLETE.
