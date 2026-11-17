# Case: city_mexico_reforma_tls (Paseo de la Reforma / Insurgentes, large boulevards with many signals, --tls.join), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-99.1720,19.4260,-99.1600,19.4330` (about 1.1 km x 780 m, central Mexico City),
retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 2.5 MB, 2005 ways, 270 `traffic_signals` tags.
Right-hand traffic, wide multi-lane boulevards with many signals.

`net.args` = `--tls.join`, which clusters close traffic-light nodes into joint signal plans. Netconvert joins them.

Structure of the reference net: 881 non-internal edges, 2715 lanes, 634 junctions, 96 traffic-light logics.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --tls.join -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou      # 300 vehicles
make_sim_refs.sh city_mexico_reforma_tls                                    # sim.args = --end 600
```
Reference run: tripinfo, FCD and statistics written by the C++ sumo, checked by `check_ref_complete.py`.
