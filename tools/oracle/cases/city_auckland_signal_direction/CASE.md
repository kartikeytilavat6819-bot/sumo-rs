# Case: city_auckland_signal_direction (Auckland CBD, left-hand traffic, bus lanes and signal direction tags), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=174.7625,-36.8495,174.7670,-36.8465` (about 400 m x 330 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.76 MB, 923 ways: 45 `traffic_signals:direction` tags
(forward and backward), 20 `bus:lanes`, 25 `turn:lanes` (incl. `turn:lanes:backward`), 111 `highway=crossing`,
2 `oneway:bicycle=no`; the highest-degree node touches 6 highway ways.

Stresses netconvert's traffic-light direction handling (`traffic_signals:direction` forward and backward on
approaches), bus lanes with `bus:lanes:forward` / `bus:lanes:backward`, and `turn:lanes` in both directions
under `--lefthand`.

`net.args` = `--lefthand`. Reference net: 871 junctions, 2391 edges, 2557 lanes.

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.
Reference: `netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml`, `make_sim_refs.sh
city_auckland_signal_direction` (sim.args = `--end 600`).
