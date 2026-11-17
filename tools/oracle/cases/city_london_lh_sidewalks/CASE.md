# Case: city_london_lh_sidewalks (net only)

Variant of `city_london_lh`: same OSM input (`../city_london_lh/input.osm`, not copied), left-hand traffic, plus
`--osm.sidewalks --osm.bike-access --osm.turn-lanes`. Net stage only; no routes.

Reference: `net_ref.sh` (C++ netconvert 1.28.0 from `~/sumo-pristine`, WSL). Ours: `run_all.sh` passes `net.args`.
