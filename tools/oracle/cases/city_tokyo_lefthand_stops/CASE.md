# Case: city_tokyo_lefthand_stops (net only)

Variant of `city_tokyo_lefthand`: same OSM input (`../city_tokyo_lefthand/input.osm`, not copied), left-hand traffic,
plus `--osm.stop-output.length 12 --osm.stop-output.length.bus 9` (public transport stop default lengths). Net only.

Reference: `net_ref.sh` (C++ netconvert 1.28.0 from `~/sumo-pristine`, WSL). Ours: `run_all.sh` passes `net.args`.
