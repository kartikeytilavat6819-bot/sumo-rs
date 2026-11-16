# jt_right_before_left — 4-arm `right_before_left` junction

Centre node `C` type `right_before_left`, arms N, E, S, W. All incoming edges use default priority, so the
rule alone decides who goes. Full connection set (12 connections). Routes: 20 vehicles in simultaneous
perpendicular pairs at 0, 6, …, 54 s.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (85 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
