# jt_t_priority — 3-arm T-junction `priority`

Centre node `C` type `priority`, arms N, E, S (no W). N and S incoming `priority=2` (main), E `priority=1`.
Connections: 6 (every incoming to each of the two other outgoing edges). Turning conflict: E→N/S
and left turns from N and S cross each other's straight path. Routes: 20 vehicles in pairs at
0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (56 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
