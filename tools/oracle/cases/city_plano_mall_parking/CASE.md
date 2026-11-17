# Case: city_plano_mall_parking (Plano shopping district, parking areas and vehicles that stop in them), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=-96.8280,33.0660,-96.8170,33.0740` (about 1.0 km x 0.9 km,
Plano TX retail area), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 1.3 MB. Right-hand traffic,
no `net.args` (default netconvert options).

Structure of the reference net: 4570 edges (grep `<edge `).

Parking areas: `generateParkingAreas.py -n ref_simple.net.xml -o parking.add.xml -p 0.15 -s 1` (pristine 1.28.0 tool),
32 `<parkingArea>` elements on road lanes (`parking.add.xml`).

Routes: 150 cars from `randomTrips.py -n ref_simple.net.xml -e 600 -p 4 --seed 1 --prefix car`. Post-processed by a small
script: for every vehicle whose route passes an edge with a parking area, every second such vehicle gets
`<stop parkingArea="..." duration="30"/>` (132 candidates, 66 stops). Result: `routes.rou`.

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
make_sim_refs.sh city_plano_mall_parking        # sim.args = --end 600 --additional-files .../parking.add.xml
```
Extra output: `--stop-output stop` (`extra_outputs.txt`, compared as `ref.stop.xml`). `--parking-output` and
`--ptstop-output` are netconvert options in 1.28, not sumo options, so they are not used here.
Reference run: tripinfo, FCD, statistics and stop output written by the C++ sumo, checked by `check_ref_complete.py`.
