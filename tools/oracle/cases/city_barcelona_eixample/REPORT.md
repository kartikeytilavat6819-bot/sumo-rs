# REPORT: city_barcelona_eixample (run 2026-11-16, ours = debug build in sumo-rs-target-x07)

| stage | verdict | detail |
|---|---|---|
| net (`compare_net.py`) | **PASS** | `SUMMARY: 0 differences in 0 groups (ref 4379 elements, cand 4379 elements)`. |
| sim | **PANIC** | our `sumo.exe` exits 1 at t~127: `thread 'main' panicked at crates\sumo_core\src\microsim\ms_vehicle.rs:4417:13: assertion failed: false`. Quitting on error. |
| sim tripinfo (partial) | **FAIL** | `vehicle 22 field arrivalSpeed: ref=13.54 cand=12.99`. Output is truncated by the abort, so later differences are not meaningful. |
| sim fcd (partial) | **FAIL** | first diff: `t=36.00 vehicle 11 field x: ref=105.71 cand=105.25`. |
| sim stat | **FAIL** | the candidate stat file is missing the `teleports`, `safety` and other sections (the run aborted before they were written). |

Context before the panic (our log): teleports on a collision, `Teleporting vehicle '15'; collision with vehicle '33'`
at t=119 and `Teleporting vehicle '57'; collision with vehicle '17'` at t=127. Whether the C++ reference produces
the same collisions was not checked.

First differences, in time order:
1. **t=36, vehicle 11 position**: `x` 105.71 (C++) vs 105.25 (ours). Earliest divergence in the FCD.
2. **t=119, collision teleports** that the reference does not produce.
3. **t=127, `ms_vehicle.rs:4417` assertion**: abort, so the sim output ends here.

The net is identical, so the problem is in the simulation path, most likely car-following or junction handling
on this dense grid. Next step: find the assertion at `ms_vehicle.rs:4417` and the first vehicle whose FCD x
diverges at t=36.
