# jt_priority — 4-arm `priority` junction

Centre node `C` (type `priority`), arms N, E, S, W (`n<arm>`, type `unregulated`). Incoming edges `<arm>_in`
with `priority=2` for N and S (main road), `priority=1` for E and W (minor). Full connection set: every
incoming to each of the other three outgoing edges (12 connections).

Routes: 20 vehicles, departures in pairs at 0, 6, …, 54 s; each pair starts on perpendicular arms, so
conflicting traffic from all arms reaches the centre together.

Binary (ours): `C:\Users\karti\sumo-rs-target-x07\debug`. Reference: C++ sumo/netconvert 1.28.0 (WSL).

| check | result |
|---|---|
| net (`compare_net.py`) | PASS — 0 differences (93 elements) |
| tripinfo | MATCH |
| FCD | MATCH |
| statistics | MATCH |

First diffs: none.
