# REPORT: city_karlsruhe_tram (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | PASS | `SUMMARY: 0 differences in 0 groups (ref 4067 elements, cand 4067 elements)` |
| sim tripinfo | FAIL | 269 entries, same count as ref. First diff: `vehicle 134 arrival ref=331.00 cand=334.00` |
| sim fcd | FAIL | first diff at `t=327.00 vehicle 128 x ref=416.65 cand=415.08` |
| sim stat | FAIL | 9 differences in 9 groups. `waitingTime ref=3.63 cand=3.42` |

First difference: vehicle 134 arrives 3 s later in ours. The FCD already differs for vehicle 128 at t=327. Not yet bisected.
