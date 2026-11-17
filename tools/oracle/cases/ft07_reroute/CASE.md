# ft07_reroute

Congestion on the `cfmodels_new` Times Square net (`ref_simple.net.xml`, copied). 20 vehicles on routes A and B, half of them a slow IDM type.

- Base (`ref.*`): device rerouting with probability 1, period 20, adaptation-steps 18, adaptation-interval 10, threads 0. `--end 200`.
- `astar` and `CH` variants: `--routing-algorithm` astar / CH with rerouting probability 1, period 20.

Refs: `bash tools/oracle/make_sim_refs.sh ft07_reroute` (C++ 1.28.0 from ~/sumo-pristine, WSL). `NEW_CASE` marks a new case.
