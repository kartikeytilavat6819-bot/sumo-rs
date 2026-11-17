# Case: real (Times Square OSM extract), simulation

Network: `ref_simple.net.xml` (from `input.osm`, netconvert reproduces it with 0 differences).
Routes: `routes.rou`, 6 vehicles, departs 0–12 s, `departSpeed="max"`, explicit edge lists
on secondary/residential edges (adjacent edges checked by from/to connectivity).

## Reference (C++ sumo 1.28.0, WSL)

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ref.tripinfo.xml --fcd-output ref.fcd.xml \
  --statistic-output ref.stat.xml --no-step-log true
```

## Ours

```
cargo build -p sumo
debug/sumo.exe -n ref_simple.net.xml -r routes.rou --tripinfo-output ts_tripinfo.xml \
  --fcd-output ts_fcd.xml --statistic-output ts_stat.xml --no-step-log true
```

## Compare

```
python tools/oracle/compare_sim.py --tripinfo ref.tripinfo.xml ts_tripinfo.xml
python tools/oracle/compare_sim.py --fcd ref.fcd.xml ts_fcd.xml
python tools/oracle/compare_stat.py ref.stat.xml ts_stat.xml
```

## Findings

- Ours runs to completion, exit 0. Only a warning: SUMO_HOME not set, XML validation disabled. No panic.
- Tripinfo: 6 entries, same as reference.
- `compare_sim --tripinfo`: MATCH. `compare_sim --fcd`: MATCH.
- `compare_stat`: `SUMMARY: 0 differences in 0 groups (ref 9 elements, cand 9 elements)`.
- Only raw-text differences are the header comment and file paths, which the comparators ignore.
