# jt_priority_stop — 4-arm `priority_stop` junction

Same geometry, edges and routes as `jt_priority`; only the centre node type is `priority_stop`
(minor-road vehicles must stop before entering). N and S incoming `priority=2`, E and W `priority=1`.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (93 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
