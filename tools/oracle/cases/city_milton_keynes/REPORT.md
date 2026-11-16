# REPORT: city_milton_keynes (gate 2a71bf88, branch worktree-agent-city-batch)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | net stage of `run_all.sh`. |
| sim (tripinfo, fcd, stat) | **PASS** | all three compared outputs match the regenerated C++ references. |

Refs were regenerated from pristine C++ 1.28.0 (`make_sim_refs.sh`, completeness checked).
The earlier net FAIL (15 roundabout/connection differences, junction `89966133`) does not reproduce on this branch;
`run_all.sh` reports the net PASS.
