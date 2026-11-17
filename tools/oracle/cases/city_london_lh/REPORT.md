# REPORT: city_london_lh (gate 2a71bf88, branch worktree-agent-city-batch)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | net stage of `run_all.sh` (`--lefthand` from `net.args`). |
| sim (tripinfo, fcd, stat) | **FAIL** | first FCD diff: `t=304.00 vehicle 146 field x: ref=982.34 cand=981.42`. First tripinfo diff: `vehicle 87 field arrivalSpeed: ref=8.95 cand=8.23`. |

Refs were regenerated from pristine C++ 1.28.0 (`make_sim_refs.sh`, completeness checked).
Not fixed here: this sim divergence belongs to the in01 work. The stat diffs are `vehicleTripStatistics` duration,
speed and timeLoss (ref 114.29 / 7.11 / 20.77 vs ours 114.14 / 7.12 / 20.65).
