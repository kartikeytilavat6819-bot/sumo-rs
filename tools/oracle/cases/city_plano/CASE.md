# Case: city_plano (US suburban arterial, Plano TX), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=-96.8320,33.0710,-96.8250,33.0780` (about 650 m x 780 m), retrieved
2026-11-16. `input.osm` is 500 KB. Right-hand traffic, `net.args` is empty.

Structure: suburban arterial corridor with signalised intersections and turn lanes. The extract has 31 traffic-light
logics in the reference net, 44 `traffic_signals` nodes and 32 `turn:lanes` tags in the OSM input, and 62
primary/secondary/tertiary ways. Reference net: 342 junctions, 1752 edges, 2305 lanes.

Network: `ref_simple.net.xml`, C++ netconvert 1.28.0: `netconvert --osm-files input.osm --simple-projection`.
Routes: `routes.rou`, `randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou`. The generator
reported 39 trips with no valid route, a warning only. The file holds 150 `<vehicle>` entries.
Sim: `sim.args` = `--end 600`.

## Reference (C++ sumo 1.28.0, WSL)

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml \
  --statistic-output ref.stat.xml --no-step-log true --end 600
```

Exit 0, 137 tripinfo entries. Teleports ("waited too long", jam and yield) occur at t=445 and t=463.

## Ours

- Net: `netconvert.exe --osm-files input.osm --simple-projection -o ours.net.xml`
- Sim: `sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output ... --fcd-output ... --statistic-output ...
  --no-step-log true --end 600`

## Compare

```
python tools/oracle/compare_net.py ref_simple.net.xml ours.net.xml
python tools/oracle/compare_sim.py --tripinfo ref.tripinfo.xml ours.tripinfo.xml
python tools/oracle/compare_sim.py --fcd ref.fcd.xml ours.fcd.xml
python tools/oracle/compare_stat.py ref.stat.xml ours.stat.xml
```

## Findings

See REPORT.md for the verdicts.
