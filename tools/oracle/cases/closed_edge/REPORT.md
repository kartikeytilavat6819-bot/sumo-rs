# Closed-edge enforcement — verdict (2026-11-16)

| Stage | Verdict | Detail |
|---|---|---|
| net | PASS | shared net with `reroute_alt` |
| sim (`ref`) | **FAIL** | first difference `v0` arrival ref 30.00, ours 31.00 |

**First difference:** `t=8.00 vehicle v0 field y: ref=63.73 ours=69.63`.

**C++ behaviour (reference):** `v0` is committed to `10 11 13`. When lane `11_0` closes (t=5), C++ does not hold it and
does not drive it through. At t=8, on reaching the junction, it reroutes onto `12_0` (`10_0 → 12_0@8 → :4_0_0 → 13_0`).
So C++ enforces the closure by rerouting the vehicle that is already en route, without any device.

**Ours:** `v0` takes `11_0` at t=8 and drives through the closed lane (`10_0 → 11_0@8 → :4_2_0 → 13_0`). The closure
has no effect on a vehicle already committed to it.

**What this proves:** closed-lane enforcement is absent in our simulator for vehicles already on a route through the
closed lane. Routing around the closure at departure is a separate mechanism and is not what this case tests.

**Side finding (not in this case's reference):** a vehicle that departs *after* the closure with an explicit route
through the closed lane (`v1` departing at t=10) makes C++ abort: `Error: Vehicle 'v1' has no valid route. No
connection between edge '10' and edge '11'`. That case was dropped from this reference because C++ produces no output
for it. It is a separate behaviour worth its own case; our behaviour there is untested.
