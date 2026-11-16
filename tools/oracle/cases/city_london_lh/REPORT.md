# REPORT: city_london_lh (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 6302 elements, cand 6302 elements)`. Our netconvert exit 0 with `--lefthand`. |
| sim tripinfo (`compare_sim.py --tripinfo`) | **FAIL** | first diff: `vehicle 66 extra in candidate`. |
| sim fcd (`compare_sim.py --fcd`) | **FAIL** | first diff: `extra timestep in candidate at time 236.00`. |
| sim stat (`compare_stat.py`) | **FAIL** | `vehicles inserted: ref=116 cand=150`; `vehicles running: ref=55 cand=0`; `waiting: ref=2 cand=0`. |

The sim stage ran our sumo on the committed C++ net (`ref_simple.net.xml`), so the sim verdict does not depend on
our netconvert.

Notes:
- The C++ reference inserts 116 of the 150 routed vehicles within 600 s; we insert all 150. The first divergence
  is a departure difference, not a movement difference: vehicle 66 exists only in our tripinfo.
- Our sumo.exe run from Windows prints `Environment variable SUMO_HOME is not set` warnings. These are the
  harness artefacts described in the Plano case, not defects.
- Not yet investigated: why C++ leaves 34 vehicles uninserted. That is the next thing to check.
