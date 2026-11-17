# jt_right_before_left_lefthand — `right_before_left` junction, left-hand traffic

Same XML as `jt_right_before_left`. `net.args` adds `--lefthand`, so netconvert mirrors the rules; the
sim reads `lefthand` from the net and needs no flag. Routes: 20 vehicles in simultaneous perpendicular
pairs at 0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (85 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
