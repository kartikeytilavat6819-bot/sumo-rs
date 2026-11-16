# The junction blocking rule: everything established so far

Open defect: on `tools/oracle/cases/junction/`, `v0` arrives at 18.00 s in our port against
20.00 s in C++, and the FCD first diverges at `t=8` (`v0` x: ref 134.09, ours 139.58 — we are
5.5 m ahead, i.e. we under-block).

## Established (do not re-derive — four agents verified these)

- `incLanes` and the `<request>` rows of the junction are **identical** to the C++ net.
- The foe set is **correct**: `v0`'s link (request 9) has foes {0,1,2,4,5,6}; index 0 is `v1`'s major link.
- `MSLink::opened` **does** consult the foe list and returns "closed" when a foe is present.
- Approach registration works: on minor links the foe entry had approaching vehicles in 287 of 295 checks.
- `setApproaching` / `removeApproaching` / `setApproachingForAllLinks` / `removePassedDriveItems` /
  `checkRewindLinkLanes` / `cleanupFurtherLanes` / `resetApproachOnReroute` — **all match C++ 1:1**,
  including the 8 call sites and the `myLFLinkLanesPrev` swap.
- C++ `MSLink::removeApproaching` is a plain `myApproachingVehicles.erase(veh)` keyed by vehicle
  identity, no `leavingTime` test (there is no separate "leaving" container).
- `MSJunction::postload_init` was not dispatching; fixed. `link_bits_test`/`response_test` were
  permissive `false` stubs; fixed.

## The quantitative fingerprint

| variant | `v0` x at t=8 |
|---|---|
| C++ reference | **134.09** |
| ours today | **139.58** (under-blocks) |
| ours with the approach record kept forever | **132.84** (over-blocks by 1.25 m) |

So the correct behaviour blocks at t=8 in a way our code does not, and blanket retention is
*almost* right but slightly too much.

## Where the difference was NOT found

- Not in removal timing (j03: erase condition and all call sites identical).
- Not in the foe set, the bit mapping, or `opened` itself (gap02).
- Not in the vehicle's own speed decision path (`planMove`, `opened`, `blockedAtTime`,
  `blockedByFoe`, `computeFoeArrivalTimeBraking`, `setApproaching`, arrival-time maths,
  Euler brake gap, emergency decel, `ignoreRed`, `ignoreFoe`) — all match (gap01).

## Remaining hypothesis

The difference is in the **data** of the blocking check — `leaveTime` / `willPass` on the
approaching record — not in when the record is erased. Our record may say "this foe leaves the
link earlier than it really does", so `blockedByFoe`'s time comparison passes when C++'s fails.

## Next step that would settle it

Instrument the **C++** build instead of ours. A debug print in
`MSLink::blockedByFoe` / `MSLink::setApproaching` for the junction, in the WSL build:

```bash
wsl -e bash -lc "cd ~/sumo-src/build && make -j8 sumo"    # after adding a fprintf guarded by an env var
```

then run the junction case with that env var and compare the `(arrivalTime, leaveTime, willPass)`
triples C++ registers for `v1` against ours at the same timesteps. Four failed attempts at
reading the code say the answer is in the numbers, not in the control flow.

## Resolution (agent j04): stale MSGlobals stubs in `ms_link.rs`

Measured both sides: C++ copy in WSL with env-guarded `SUMO_TRACE_LINK` prints; Rust with the same prints (since reverted).

**Registration triples do not differ on `v1`'s major link.** Every `setApproaching`/`removeApproaching`
line for `v1` and the foe links is identical until t=8 s. The first differing trace line is `v0`'s
*own* record at 8000 ms: arrival 7295 in both, but leave speed 4.536474 (C++) vs 14.988618 (ours),
so `leaveTime` 10661 vs 9296. The data hypothesis was wrong; the decision upstream of the record differs.

Chain of cause, from the traces at 8000 ms:

1. `MSVehicle::adaptToJunctionLeader` site 7 (`vsafeLeader`, MSVehicle.cpp:3474) fires in C++ for `v0`
   with value 4.536474, and never in ours. It needs a link leader.
2. C++ `MSLink::getLeaderInfo` returns a leader for `v0`:
   `TRACE_LL 8000 :-137721_9_0->-103224#2_0 ego=v0 lead=v1 gap=-9.776157 inWay=0`. Ours returns none, at any time.
3. Candidate inputs for `v1` are identical on both sides (`cI=0 inW=0 wP=1 pCP=0 eCP=0 lb=-3.947767 lbd=11.150106`).
   Both reach the waiting-time gate, where they diverge:
   ```
   C++:  TRACE_GATE 8000 :-137721_9_0->-103224#2_0 ego=v0 foe=v1 wt=0 gate=9223372036854775807
   ours: TRACE_GATE 8000 :-137721_9_0->-103224#2_0 ego=v0 foe=v1 wt=0 gate=0
   ```
   `wt < gate` is true in C++ (the blocker is recorded) and false in ours, so no leader is pushed.

Cause: `ms_link.rs` had six shortcut stubs, left over from before `MSGlobals` was ported, returning constants:
`g_ignore_junction_blocker() -> 0` (the one that matters), `g_compute_lc`, `g_sublane`, `g_lefthand`,
`g_use_meso_sim`, `g_using_internal_lanes`. They now call `microsim::ms_globals`.

| check | before | after |
|---|---|---|
| junction FCD vehicle rows vs `ref.fcd.xml` | first diff at t=8 (x 139.58 vs 134.09) | 0 diff rows (84/84 identical) |
| junction `v0` arrival | 18.00 | 20.00 |
| `run_all` junction sim | FAIL | PASS |
| `run_all` roundabout sim | FAIL | PASS |
| `run_all` real_dense / signal sim | FAIL | FAIL (identical output) |
