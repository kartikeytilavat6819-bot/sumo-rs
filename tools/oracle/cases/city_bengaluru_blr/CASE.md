# Case: city_bengaluru_blr (Bengaluru MG Road block, left-hand traffic), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=77.6060,12.9725,77.6130,12.9790`, retrieved 2026-11-16 through Kaggle kernel `kartikey5656/sumo-rs-osm-dl2` (Overpass is blocked on this machine). `input.osm` is 1.5 MB. `net.args` = `--lefthand`.

Reference (C++ sumo and netconvert 1.28.0 from `~/sumo-pristine`, WSL):
```
netconvert --osm-files input.osm --simple-projection --lefthand -o ref_simple.net.xml
randomTrips.py -n ref_simple.net.xml -e 600 -p 2 --seed 1 -r routes.rou
sumo -n ref_simple.net.xml -r routes.rou --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml --statistic-output ref.stat.xml --no-step-log true --end 600
```
Each C++ run was checked for exit 0, closing tags, and FCD reaching t=600. `sim.args` = `--end 600`.

Compare: see `city_plano/CASE.md`. Verdicts are in REPORT.md.
