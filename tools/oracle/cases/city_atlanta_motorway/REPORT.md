# REPORT: city_atlanta_motorway (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 17671 elements, cand 17671 elements)`. Our netconvert exit 0. |
| sim tripinfo (`compare_sim.py --tripinfo`) | **FAIL** | first diff: `vehicle 81 extra in candidate`. |
| sim fcd (`compare_sim.py --fcd`) | **FAIL** | first diff: `extra timestep in candidate at time 250.00`. |
| sim stat (`compare_stat.py`) | **FAIL** | `vehicles inserted: ref=125 cand=150`; `vehicles running: ref=58 cand=1`; `vehicleTripStatistics count: ref=67 cand=149`. |

The sim stage ran our sumo on the committed C++ net, so the sim verdict does not depend on our netconvert.

Notes:
- Same shape as `city_london_lh`: the C++ reference inserts 125 of 150 vehicles, we insert all 150. The first
  divergence is vehicle 81 existing only in our output.
- Our net matches C++ exactly (17671 elements), so the sim divergence is on the simulation side, not the net.
- Not yet investigated: the cause of the departure difference and the reference's 58 running vehicles versus our 1.
