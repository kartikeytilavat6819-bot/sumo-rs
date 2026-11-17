# Case: city_london_lh (left-hand traffic, central London), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-0.1340,51.5010,-0.1270,51.5070` (about 500 m x 660 m, near Trafalgar
Square / Whitehall), fetched 2026-11-16 through a Kaggle kernel (Overpass was unreachable from the dev machine).
`input.osm` is 2.9 MB. Left-hand traffic, `net.args` = `--lefthand`.

Structure: dense left-hand urban grid with one-way streets, bus lanes and traffic signals. This case checks that
left-hand generation and the mirrored junction logic agree with C++.

Network: `ref_simple.net.xml`, C++ netconvert 1.28.0:
`netconvert --osm-files input.osm --simple-projection --lefthand`.
Routes: `routes.rou`, `randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou`.
Sim: `sim.args` = `--end 600`.

## Reference (C++ sumo 1.28.0, WSL)

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml \
  --statistic-output ref.stat.xml --no-step-log true --end 600
```

## Ours

- Net: `netconvert.exe --osm-files input.osm --simple-projection --lefthand -o ours.net.xml`
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
