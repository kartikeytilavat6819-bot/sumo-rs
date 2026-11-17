# jt_traffic_light_right_on_red — 4-arm `traffic_light_right_on_red` junction

Centre node `C` type `traffic_light_right_on_red` (netconvert builds the default tlLogic), arms N, E, S, W,
full connection set (12). Routes: 20 vehicles in simultaneous perpendicular pairs at 0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (106 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
