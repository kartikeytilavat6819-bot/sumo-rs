# Case: real_dense (Times Square OSM extract, congested simulation)

Network: `ref_simple.net.xml` and `input.osm`, copied from `tools/oracle/cases/real/` (commit 4ef42da).
Routes: `routes.rou`, **48 vehicles** over 6 route templates (8 vehicles each), departures
`i * 1.25` s for i = 0..47 (0.00–58.75 s), `departSpeed="max"`. No random departures.
Routes cross at signalised junctions 42439984, 42439990, 42428297 and 6598535188 so vehicles conflict and queue.
Only `highway.secondary` / `highway.residential` edges are used. `highway.service` edges are avoided
because passenger vehicles cannot use them (a disallowed edge is reported as "No connection").

## Reference (C++ sumo 1.28.0, WSL)

```
wsl -e bash -lc 'D=/mnt/c/Users/karti/sumo-rs/<worktree>/tools/oracle/cases/real_dense; \
  SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo -n $D/ref_simple.net.xml -r $D/routes.rou \
  --tripinfo-output $D/ref.tripinfo.xml --fcd-output $D/ref.fcd.xml \
  --statistic-output $D/ref.stat.xml --no-step-log true --end 125'
```

Reference: exit 0, 40 of 48 vehicles arrived by t=125 s. Horizon moved from 120 to 125 s, see `REPORT.md`.

## Ours

```
cargo build -p sumo   # CARGO_TARGET_DIR=C:\Users\karti\sumo-rs-target-ts02
C:\Users\karti\sumo-rs-target-ts02\debug\sumo.exe -n ref_simple.net.xml -r routes.rou \
  --tripinfo-output ours.tripinfo.xml --fcd-output ours.fcd.xml \
  --statistic-output ours.stat.xml --no-step-log true --end 125
```

## Compare

```
python tools/oracle/compare_sim.py --tripinfo ref.tripinfo.xml ours.tripinfo.xml
python tools/oracle/compare_sim.py --fcd ref.fcd.xml ours.fcd.xml
python tools/oracle/compare_stat.py ref.stat.xml ours.stat.xml
```

Note: `compare_sim.py` exits on the first difference, so only the first diff per comparator is reported.

## Findings

- Ours runs to completion, exit 0, no panic. Only warning: SUMO_HOME not set, XML validation disabled.
- Tripinfo: **ours 24 entries, reference 38**. 20 vehicles never arrive in ours within 120 s:
  v02 v03 v04 v08 v09 v10 v14 v15 v16 v20 v21 v22 v26 v28 v32 v34 v38 v40 v44 v46.
- `compare_sim --tripinfo`: first diff `vehicle v19 field timeLoss: ref='2.60' cand='2.59'`.
- `compare_sim --fcd`: first diff `t=40.00 vehicle v23 field x: ref='334.63' cand='333.56'`.
- `compare_stat`: `SUMMARY: 11 differences in 11 groups (ref 9 elements, cand 9 elements)`.
  Notable: `totalDepartDelay` ref 279.50 vs ours 13.00; `totalTravelTime` ref 1291.00 vs ours 619.00;
  `waitingTime` ref 13.61 vs ours 0.00; `timeLoss` ref 19.41 vs ours 2.94.

The light case (`real/`) matches exactly. Under this load it diverges: 20 of 38 reference
arrivals are missing from ours, and waiting/delay statistics differ by a large margin.
Cause not yet investigated.
