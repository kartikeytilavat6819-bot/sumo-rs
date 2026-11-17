# Case: city_istanbul_bosphorus_layers (Istanbul, Bosphorus bridge approach at Ortaköy, layered roads), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=29.0290,41.0410,29.0400,41.0500` (about 1.0 km x 1.0 km, around the
15 Temmuz Şehitler Köprüsü, the Bosphorus Bridge), retrieved 2026-11-17 from WSL with a User-Agent header.
`input.osm` is 1.0 MB. Right-hand traffic. Bridges, viaducts and tunnels carry `layer` tags (1 and up).

`net.args` = `--osm.layer-elevation 5`. Each `layer` level raises the road by 5 m (relative elevation from the layer tags),
so bridges and tunnels get a z-offset in the net. Everything else uses netconvert defaults.

Structure of the reference net (grep of `ref_simple.net.xml`, edges include internal ones):
107 edges, 59 junctions, 10 traffic-light logics. Several edges carry a non-zero z coordinate from the layer tags.

Routes: 150 cars, one file (randomTrips, seed 1, period 4 s, 600 s):
```
randomTrips.py -n ref_simple.net.xml -e 600 -p 4 --seed 1 --prefix car -r cars.rou.xml
```

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.layer-elevation 5 -o ref_simple.net.xml
make_sim_refs.sh city_istanbul_bosphorus_layers                            # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
