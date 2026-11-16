# Case: city_atlanta_motorway (motorway interchange with on/off ramps), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-84.3880,33.7680,-84.3820,33.7740` (about 500 m x 660 m around the
Downtown Connector / I-75/85 interchange, Atlanta GA), fetched 2026-11-16 through a Kaggle kernel.
`input.osm` is 2.6 MB. Right-hand traffic, `net.args` is empty.

Structure: motorway with on/off ramps, lane merges and diverges, and zipper-like merge sections. This case
stresses lane-change and merge logic, which the urban cases do not.

Network: `ref_simple.net.xml`, C++ netconvert 1.28.0: `netconvert --osm-files input.osm --simple-projection`.
Routes: `routes.rou`, `randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou`.
Sim: `sim.args` = `--end 600`.

## Reference (C++ sumo 1.28.0, WSL)

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml \
  --statistic-output ref.stat.xml --no-step-log true --end 600
```

## Ours

- Net: `netconvert.exe --osm-files input.osm --simple-projection -o ours.net.xml`
- Sim: `sumo.exe -n ref_simple.net.xml -r routes.rou ... --end 600`

## Compare

```
python tools/oracle/compare_net.py ref_simple.net.xml ours.net.xml
python tools/oracle/compare_sim.py --tripinfo ref.tripinfo.xml ours.tripinfo.xml
python tools/oracle/compare_sim.py --fcd ref.fcd.xml ours.fcd.xml
python tools/oracle/compare_stat.py ref.stat.xml ours.stat.xml
```

## Findings

See REPORT.md for the verdicts.
