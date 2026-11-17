# jt_left_before_right — 4-arm `left_before_right` junction

Centre node `C` type `left_before_right`, arms N, E, S, W, default priorities, full connection set (12).
Routes: 20 vehicles in simultaneous perpendicular pairs at 0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (85 elements) |
| tripinfo | **FAIL** — first diff `v19` arrival: ref 89.00, ours 110.00 |
| FCD | **FAIL** — first diff t=61.00 `v18` y: ref 91.27, ours 91.94 |
| statistics | **FAIL** — `vehicleTripStatistics` duration: ref 51.80, ours 51.25 |

Waiting differs from the first conflicting departures on: C++ `v16` waits 22 s, ours 17 s; C++ `v18`
waits 3 s, ours 0 s; C++ `v19` waits 2 s, ours 18 s. Ours yields to the wrong vehicles in the
left-before-right ordering. Not investigated further (sim verdict is the finding).
