# REPORT: city_plano (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 7378 elements, cand 7378 elements)`. Our netconvert exit 0. |
| sim tripinfo (`compare_sim.py --tripinfo`) | **PASS** | MATCH, 137 entries, against the committed C++ reference. |
| sim fcd (`compare_sim.py --fcd`) | **PASS** | MATCH. |
| sim stat (`compare_stat.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 9 elements, cand 9 elements)`. |

The sim stage ran our sumo on the committed C++ net (`ref_simple.net.xml`), as `run_all.sh` does, so the sim
verdict does not depend on our netconvert. Our net is identical to C++ on all 7378 elements.

No first differences to report.
