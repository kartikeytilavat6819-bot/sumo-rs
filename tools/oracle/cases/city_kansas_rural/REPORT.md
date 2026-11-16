# REPORT: city_kansas_rural (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

Design note: the extract has 126 junctions and 504 real edges, not the "few junctions" the case was meant to have.

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | PASS | `SUMMARY: 0 differences in 0 groups (ref 6946 elements, cand 6946 elements)` |
| sim tripinfo | FAIL | 244 entries vs 243 in ref. First diff: `vehicle 14 arrival ref=142.00 cand=141.00` |
| sim fcd | FAIL | first diff at `t=127.00 vehicle 56 x ref=4380.31 cand=4380.09` |
| sim stat | FAIL | 10 differences in 10 groups. `waitingTime ref=6.34 cand=6.13` |

First difference: vehicle 14 arrives 1 s early in ours. Not yet bisected.
