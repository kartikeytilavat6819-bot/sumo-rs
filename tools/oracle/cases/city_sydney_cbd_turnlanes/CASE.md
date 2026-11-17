# Case: city_sydney_cbd_turnlanes (Sydney CBD, Western Distributor motorway junction, left-hand traffic), net + simulation

Source: OSM API `https://api.openstreetmap.org/api/0.6/map?bbox=151.2020,-33.8730,151.2060,-33.8695` (about 330 m x 390 m),
retrieved 2026-11-17 with a User-Agent header. `input.osm` is 1.7 MB, 562 ways, 34 `motorway_link` ways, 19 ways with
`bus:lanes` or `turn:lanes`.

Stresses netconvert's OSM import in ways the other cities do not: dense motorway_link ramp weaving under the
Western Distributor and Cahill Expressway, `turn:lanes` and `bus:lanes` on the arterials, and `--lefthand` on a
motorway-ramp layout (Bathurst, Harbour and Kent streets).

`net.args` = `--lefthand`. Reference net: 555 `<junction>`, 1265 `<edge>`, 1658 `<lane>` elements. netconvert removes 3 pt stops that are not
assigned to an edge (warning, expected).

Routes: 120 cars, `randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 5 --seed 1 --prefix car -r routes.rou
sumo -n ref_simple.net.xml -r routes.rou --end 600 ...   # via make_sim_refs.sh (NEW_CASE), sim.args = --end 600
```
