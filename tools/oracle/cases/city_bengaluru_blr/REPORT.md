# REPORT: city_bengaluru_blr (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | PASS | `SUMMARY: 0 differences in 0 groups (ref 9555 elements, cand 9555 elements)` |
| sim tripinfo | FAIL | 219 entries vs 217 in ref. First diff: `vehicle 47 arrivalSpeed ref=9.35 cand=9.94`. Then `t=462 vehicle 125 x ref=944.91 cand=944.69`. `vehicleTripStatistics waitingTime ref=16.29 cand=16.35` |
| sim fcd | FAIL | first diff at `t=462.00 vehicle 125 x` |
| sim stat | FAIL | 11 differences in 11 groups |

First difference: vehicle 47 arrival speed, 9.35 vs 9.94. Not yet bisected.
