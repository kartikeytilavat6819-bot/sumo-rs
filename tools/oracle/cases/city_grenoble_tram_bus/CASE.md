# Case: city_grenoble_tram_bus (Grenoble city centre, bus stops with riding persons), net + simulation

Source: OSM via OpenStreetMap API `/api/0.6/map`, `bbox=5.7235,45.1880,5.7275,45.1915` (about 0.3 km x 0.4 km, Place Victor
Hugo and Hotel de Ville area), retrieved 2026-11-17 from WSL with a User-Agent header. `input.osm` is 5.0 MB.
Right-hand traffic, no `net.args` (default netconvert options).

Public transport: netconvert `--ptstop-output ptstops.add.xml` (pristine 1.28.0) gives 8 `busStop` elements and no
`tramStop`. So the case has bus traffic only, not a tram line. The OSM stop `lines` attributes were removed from
`ptstops.add.xml`, because they would exclude the synthetic lines from most stops.

Routes (`routes.rou`, built by a small script with sumolib): 2 bus lines L1 and L2, each through 4 stops connected by
shortest paths. Each line has 3 buses (`departSpeed=max`, 15 s stops). 4 persons per line walk to a stop and ride to
the next one (`<walk from=... busStop=...>` then `<ride busStop=... lines=...>`). Walk edges are the nearest
pedestrian-allowed edge to the stop, because the stop lane's split edge does not allow pedestrians.

Reference net: 960 edges (grep `<edge `).

Reference (C++ netconvert and sumo 1.28.0, WSL, `~/sumo-pristine`):
```
netconvert --osm-files input.osm --simple-projection --ptstop-output ptstops.add.xml -o ref_simple.net.xml
make_sim_refs.sh city_grenoble_tram_bus        # sim.args = --end 600 --additional-files .../ptstops.add.xml
```
Extra outputs: `--stop-output stop` and `--personinfo-output personinfo` (`extra_outputs.txt`). The reference
personinfo has 4 completed rides and the stop output has 16 `stopinfo` records.
Reference run: tripinfo, FCD, statistics, stop and personinfo written by the C++ sumo, checked by `check_ref_complete.py`.
