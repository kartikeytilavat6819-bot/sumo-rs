# Case: city_karlsruhe_tram (Karlsruhe city centre, tram tracks and bus stops), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=8.3995,49.0080,8.4050,49.0115`, retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-dl2` (Overpass is blocked on this machine). `input.osm` is 2.5 MB and has `railway=tram` ways. Right-hand traffic, `net.args` is empty.

Reference (C++ sumo and netconvert 1.28.0 from `~/sumo-pristine`, WSL):
```
netconvert --osm-files input.osm --simple-projection -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou
sumo -n ref_simple.net.xml -r routes.rou --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true --end 600
```
Each C++ run was checked for exit 0, closing tags, and FCD reaching t=600. `sim.args` = `--end 600`.

Compare: see `city_plano/CASE.md`. Verdicts are in REPORT.md.
