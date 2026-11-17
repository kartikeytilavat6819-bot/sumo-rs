# REPORT: city_barcelona_eixample (gate 2a71bf88, branch worktree-agent-city-batch)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | net stage of `run_all.sh`. |
| sim (tripinfo, fcd, stat) | **PASS** | all three compared outputs match the regenerated C++ references. |

Refs were regenerated from pristine C++ 1.28.0 (`make_sim_refs.sh`, completeness checked).
The earlier PANIC at `ms_vehicle.rs:4417` and the t=36 FCD diff were from an older build and are superseded; the
current sim matches (cause of the change not separately measured).
