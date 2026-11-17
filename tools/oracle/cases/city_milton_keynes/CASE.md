# Case: city_milton_keynes (roundabout-heavy area, UK new town), net + simulation

Source: OSM via Overpass `/api/map?bbox=-0.7560,52.0410,-0.7480,52.0470` (about 650 m x 670 m, Milton Keynes
central grid roundabouts), fetched 2026-11-16 through a Kaggle kernel. `input.osm` is 2.1 MB. Built without `--lefthand` (`net.args` is
empty), so the net is mirrored to right-hand traffic on both sides. This is a deliberate choice, not a UK-mirror test.

Structure: a grid of roundabouts with several arms each, which is the roundabout topology the `roundabout` case
covers only in miniature.

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
