# jt_unregulated — 4-arm `unregulated` junction

Centre node `C` type `unregulated`, arms N, E, S, W, default priorities, full connection set (12).
Routes: 20 vehicles in simultaneous perpendicular pairs at 0, 6, …, 54 s. No right-of-way rule applies.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (85 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
