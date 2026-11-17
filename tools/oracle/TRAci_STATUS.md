# TraCI status (live control path)

Measured 2026-11-16 by `tools/oracle/cases/traci_basic/client.py` against C++ SUMO 1.28.0 (`~/sumo-pristine/bin/sumo`, WSL).
The simulation oracle (`run_all.sh`) does **not** exercise TraCI at all, so a green scoreboard says nothing
about this path. Run it with:

```
CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-x07 python tools/oracle/cases/traci_basic/client.py --port 8853
```

The script prints the binary it is using and shouts `STALE BINARY` if any `.rs` is newer. Two different
defaults exist across this harness (`sumo-rs-target` here, `sumo-rs-target-main` in `run_all.sh`), and
measuring the wrong one produced a false **40/62** reading on the night that check was added.

## Score

**290 of 294 comparisons match** (junction 146/147, signal 144/147; 2026-11-16, port 8853).

- Base probes (106 per case, 212 total, unchanged set): **210/212**.
- Effect probes (41 per case, added 2026-11-16, see below): **80/82**.

`STALE BINARY` was raised: the worktree checkout is newer than `C:\Users\karti\sumo-rs-target-x07\debug\sumo.exe`,
so these numbers measure that binary, not necessarily HEAD. Re-measure after a build.

Every row that was open in the previous score (`getLeader(v0, 200)`, `subscribeContext(v0, speed)` server crash,
`convertRoad`, `getDistance2D`, `findRoute`, `getJunctionFoes`) now **matches** on both cases with this binary.
Their earlier causes are not re-verified; re-check them after a rebuild before relying on them.

## Effect probes (new)

41 probes per case, all prefixed `effect:` in `client.py`. Each sets something, runs 20-50 steps, then
compares a state summary: `(vehicle, position, speed, lane)` for every vehicle, arrived count, arrived IDs;
or a per-step trace (traffic light state/phase/next switch); or a full subscription stream.
Coverage per probe is limited, see the gaps below.

## Mismatches (4 of 294; first difference per row)

| Probe | C++ | Ours | First differing step / cause |
|---|---|---|---|
| base, junction + signal: `getVersion` | `tuple (22, 'SUMO 1.28.0')` | `tuple (22, 'SUMO v1.28.0-0000000000')` | Whole value. Version string format differs. Not traced. |
| signal effect: `setLaneChangeMode(v0,0)` + `changeLane(v0, idx^1, 5)` +30 steps | step 12: `vfx` on lane `324752522_1` | step 12: `vfx` stays on `324752522_0` | Commanded lane change is not applied with lane change mode 0. Not traced further. |
| signal effect: `setRedYellowGreenState` with wrong length (`"G"` for 5 links) +5 | `FatalTraCIError Connection closed by SUMO` (the C++ server dies) | `TraCIException Mismatching phase size in tls '1802245009', program 'online'.` | C++ side dies, ours answers with the error. C++ error text unknown. Reproduced standalone. |

Grouped by cause: 1 version string format, 1 lane change not applied on our side, 1 C++ server crash (ours is the
documented behaviour; the C++ error text cannot be compared).

## Coverage gaps (not mismatches)

- **Junction effect probes are mostly empty.** All 5 junction vehicles have arrived by t≈40
  (`simulationStep(40)` shows 5 arrived), and the junction has no traffic lights (`'none'`). Only ~6 of 41 junction
  effect probes compare live vehicle state; the 10 traffic-light effect probes are all `Traffic light 'none' is not known`
  on both sides. The junction `146/147` is inflated by this. Use a longer junction route before trusting junction effect rows.
- **Signal effect probes are meaningful.** Base `v0` is gone after step 11 in the signal case, so effect probes spawn
  their own vehicle `vfx` (route 0, added at t=10, live at t=11).
- The junction changeLane effect probe was empty, so the lane change divergence above cannot be checked there.
- `vehicle.subscribeContext` effect probes use ranges 50.0 and 100.0 (20-step streams). Base probes still use range `0.0`.
- Base `vehicle.changeLane(v0)` and `vehicle.changeTarget(v0)` rows in the signal case return `Vehicle 'v0' is not known`
  on both sides, so they match without testing anything.

## Fixed this session, in order of user impact

1. `simulation.getTime` **segfaulted**, a bad cast in `Simulation::handle_variable`; now generic over a
   `SimVariableWrapper` trait with no cast, and answers `10.0`.
2. `subscribe(nope)` **killed the server**, the executor panicked outside any `catch_unwind`; it now returns
   C++'s error status and the connection survives.
3. `Vehicle::handleVariable` was a stub; all vehicle getters and `setSpeed` now work.
4. `VariableWrapper` was missing; every lane and traffic-light getter failed with *pure virtual
   VariableWrapper*; now ported (23 `wrap*` virtuals, sink dispatch).
5. `simulation.getLoadedNumber` returned 0; `MSNet::inform_vehicle_state_listener` called the base
   `VehicleStateListener` no-op (no vtable); fixed by pointer-identity dispatch to the server and `Helper`.
   Other listeners (vehicle, devices, rail, polygon) still hit the no-op. That is marked `shortcut:` and is
   the same missing-vtable class that caused seven defects in the simulation core.
6. `vehicle.changeTarget` needed the router provider; works now.
7. `setRoute`'s index-out-of-bounds panic; the validator read the *current* route instead of the one being
   checked.

## Known risks

- The panic hook still prints a backtrace line to stderr for caught panics; harmless but noisy.
- `vehicle.add` leaks its parameter block when it fails.
- Lane change with lane change mode 0 is not applied (see mismatches); the cause is not traced.
