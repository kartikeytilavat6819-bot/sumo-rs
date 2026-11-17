# Case: city_hongkong_lefthand_grid (Mong Kok high-rise grid, left-hand traffic), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=114.1700,22.3170,114.1740,22.3200` (about 380 m x 330 m, Mong Kok),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.6 MB, 580 ways, 182 `traffic_signals` tags.
Dense high-rise grid with many lanes and short blocks.

`net.args` = `--lefthand` (Hong Kong drives on the left). The reference netconvert gets the same flag.

Structure of the reference net: 503 non-internal edges, 1089 lanes, 371 junctions, 62 traffic-light logics, no roundabout.
Netconvert warns that some OSM traffic lights control no links and are not built (7 such messages).

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_hongkong_lefthand_grid                                # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
