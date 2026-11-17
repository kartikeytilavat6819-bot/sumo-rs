# jt_allway_stop — 4-arm `allway_stop` junction

Centre node `C` type `allway_stop`, arms N, E, S, W, default priorities, full connection set (12).
Routes: 20 vehicles in simultaneous perpendicular pairs at 0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (85 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
