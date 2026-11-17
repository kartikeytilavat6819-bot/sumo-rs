# Case: city_stelvio_pass_grades (Stelvio pass road, steep grades, trucks, emission output), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=10.4450,46.5240,10.4700,46.5330` (about 2.0 km x 1.0 km, the
Stelvio pass road hairpins, Italy), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 1.9 MB.
Right-hand traffic.

`net.args` = `--osm.elevation`. The OSM extract has 12 `ele` tags. netconvert gives heights to junctions, for example
z=1377 to z=2758 m between nearby junctions, so the grades are real but sparse (only 3 `z=` attributes in
`ref_simple.net.xml`). Reference net: 1112 edges.

Routes: 75 trucks from `randomTrips.py -n ref_simple.net.xml -e 600 -p 8 --seed 1 --prefix truck --vehicle-class truck
--vclass truck`. The vType is `truck_truck` with `emissionClass="HBEFA3/HDV"` (`HBEFA3/HDV_R_40t` is not a 1.28 class).

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --osm.elevation -o ref_simple.net.xml
make_sim_refs.sh city_stelvio_pass_grades        # sim.args = --end 600
```
Extra output: `--emission-output emission` (`extra_outputs.txt`, compared as `ref.emission.xml`).
Reference run: tripinfo, FCD, statistics and emission written by the C++ sumo, checked by `check_ref_complete.py`.
