# Case: city_munich (central Munich old-town grid), net + simulation

Source: OSM via Overpass `/api/map`, `bbox=11.5785,48.1328,11.5855,48.1374` (about 520 m x 510 m, centred on the
Marienplatz old town), retrieved 2026-11-16. `input.osm` is 2.63 MB. The first, 600 m bbox returned 3.54 MB, over the
3 MB budget, so it was shrunk. The public Overpass mirror returned 502/406 pages on several attempts; the
successful request used an explicit User-Agent.

Structure: dense grid of short streets (812 junctions, 2159 edges, 2360 lanes in the reference net), 33 traffic-light
logics. Roundabouts were not counted. `net.args` is empty (right-hand traffic).

Network: `ref_simple.net.xml`, C++ netconvert 1.28.0: `netconvert --osm-files input.osm --simple-projection`.
Routes: `routes.rou`, `randomTrips.py -n ref_simple.net.xml -e 300 -p 2 --seed 1 -r routes.rou`. The generator
reported 28 trips with no valid route, a warning only. The file holds 150 `<vehicle>` entries.
Sim: `sim.args` = `--end 600`.

## Reference (C++ sumo 1.28.0, WSL)

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml \
  --statistic-output ref.stat.xml --no-step-log true --end 600
```

Exit 0, 94 tripinfo entries. Warnings are teleports ("waited too long") at t=509/512 for vehicle 39.

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

- Net: **PANIC** in our netconvert, `crates\sumo_core\src\netbuild\nbpt_line_cont.rs:307`:
  `NBPTLineCont::fixBidiStops: shortcut, needs NBPTStop bidi links and router operations`. C++ succeeds.
- Sim (on the committed C++ net): **PANIC** in our sumo, `crates\sumo_core\src\microsim\traffic_lights\ms_traffic_light_logic.rs:68`:
  `MSTrafficLightLogic::getCurrentPhaseDef is pure virtual; call it on a concrete traffic light logic`. C++ exits 0.
  Not diagnosed. C++ netconvert warned about three traffic lights with no links (`1955028`, `278690117`,
  `293544170`), so a light without a program is the first suspect, but this has not been verified.
- No tripinfo, FCD or statistics were compared, because our sumo produced no output.
