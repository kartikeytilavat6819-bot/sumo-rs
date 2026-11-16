# TraCI status (live control path)

Measured 2026-11-16 by `tools/oracle/cases/traci_basic/client.py` against C++ SUMO 1.28.0 in WSL.
The simulation oracle (`run_all.sh`) does **not** exercise TraCI at all, so a green scoreboard says nothing
about this path. Run it with:

```
CARGO_TARGET_DIR=/c/Users/karti/sumo-rs-target-x07 python tools/oracle/cases/traci_basic/client.py
```

The script now prints the binary it is using and shouts `STALE BINARY` if any `.rs` is newer — two different
defaults exist across this harness (`sumo-rs-target` here, `sumo-rs-target-main` in `run_all.sh`), and
measuring the wrong one produced a false **40/62** reading on the night that check was added.

## Score

**201 of 212 probes match C++ 1.28.0** (junction 100/106, signal 101/106; 2026-11-16, port 8823, `STALE BINARY` warning raised). The original 62 probes (31 per case) now match 62/62. The 75 probes added for the Unity client (75 per case, 150 comparisons) match 139/150. Before this, 58 of 62 matched (junction 29/31, signal 29/31). Up from 9/32 at the start of the
session, after `VariableWrapper`, `Vehicle::handleVariable`, the wrapper sink, the panic-to-error-status
conversion and a null-RNG fix in `MSVehicleType::computeChosenSpeedDeviation` (C++ allows a null `SumoRNG*`
and means "default generator"; we dereferenced it and panicked, which made `vehicle.add` unusable).

Probes run in separate sessions, so one crash costs only its own row.

## Remaining mismatches (11 of 212; first difference per row)

Both rows that were open in the previous score (`vehicle.setRoute(v0, current)`, `vehicle.getJunctionFoes(v0)`)
now **match** on both cases: the same `Route replacement failed` text and `tuple ()`. Their earlier causes
(`replace_route_edges` only warns; `LinkBits` is `()`) are still in the code, so these probes just stopped
distinguishing them. Re-check before relying on them.

Measured with the shared binary `C:\Users\karti\sumo-rs-target-x07\debug\sumo.exe`. The script printed
`STALE BINARY` (a `.rs` in the worktree is newer than the binary), so these numbers may measure older code.

| Probe | C++ | Ours | Cause |
|---|---|---|---|
| junction: `vehicle.getLeader(v0, 200)` | `tuple ('v3', 121.716…)` | `None` | with `dist=200` ours finds no leader; the `dist=0` probe matches (cause not yet traced) |
| junction: `vehicle.subscribeContext(v0, [SPEED])` + step | `dict {'v0': {64: 8.160…}}` | `FatalTraCIError: Connection closed by SUMO` (server dies) | not yet traced; the server dies on the vehicle-context subscription (the panic message was not found in `%TEMP%\traci_ours_*.log`) |
| signal: `vehicle.subscribeContext(v0, [SPEED])` + step | `dict {}` | `FatalTraCIError: Connection closed by SUMO` (server dies) | same crash as above |
| junction + signal: `simulation.convertRoad(x, y)` | junction `tuple (':-137721_0', 8.800…, …)`; signal `tuple ('324752524', 41.618…, …)` | `shortcut: MSLane R-tree is not ported yet, cannot search lanes in range` (`libsumo/helper.rs:1255`) | MSLane R-tree (lane search in range) not ported |
| junction + signal: `simulation.getDistance2D(x, y, x+3, y+4)` | `float 5.0` | same R-tree shortcut | same R-tree cause |
| junction: `simulation.findRoute(edge, v0 last).edges` | `tuple ('-103224#1', '-103224#2', '-103…')` | `shortcut: Simulation::findRoute needs MSVehicleControl::buildVehicle and MSRoutingEngine` (`libsumo/simulation.rs:797`) | MSRoutingEngine / buildVehicle not ported |
| signal: `simulation.findRoute(edge, v0 last).edges` | `tuple ()` | same findRoute shortcut | same findRoute cause |
| junction + signal: `simulation.findRoute(nope, edge)` | `TraCIException Unknown from edge 'nope'` | same findRoute shortcut (panic, not the C++ error) | same findRoute cause; the edge check never runs |

Grouped by cause: 4 rows unported MSLane R-tree, 4 rows unported findRoute, 2 rows server crash on
`subscribeContext`, 1 row `getLeader` with a distance.

## Coverage gaps (not mismatches)

- In the **signal** case the vehicle `v0` is gone after the prelude plus 11 steps. Every "effect" probe
  (`edge.setMaxSpeed` + getSpeed, `lane.setMaxSpeed` + getSpeed, `slowDown`, `moveTo`, `changeLane`) returns
  `Vehicle 'v0' is not known` on both sides, so they match without testing anything. Use a different vehicle
  or a longer-lived one in that case before trusting those rows.
- `vehicle.subscribeContext` uses range `0.0`, because the Python API requires a range. A real no-range call
  was not tested.

## Fixed this session, in order of user impact

1. `simulation.getTime` **segfaulted** — a bad cast in `Simulation::handle_variable`; now generic over a
   `SimVariableWrapper` trait with no cast, and answers `10.0`.
2. `subscribe(nope)` **killed the server** — the executor panicked outside any `catch_unwind`; it now returns
   C++'s error status and the connection survives.
3. `Vehicle::handleVariable` was a stub — all vehicle getters and `setSpeed` now work.
4. `VariableWrapper` was missing — every lane and traffic-light getter failed with *pure virtual
   VariableWrapper*; now ported (23 `wrap*` virtuals, sink dispatch).
5. `simulation.getLoadedNumber` returned 0 — `MSNet::inform_vehicle_state_listener` called the base
   `VehicleStateListener` no-op (no vtable); fixed by pointer-identity dispatch to the server and `Helper`.
   Other listeners (vehicle, devices, rail, polygon) still hit the no-op — that is marked `shortcut:` and is
   the same missing-vtable class that caused seven defects in the simulation core.
6. `vehicle.changeTarget` needed the router provider; works now.
7. `setRoute`'s index-out-of-bounds panic — the validator read the *current* route instead of the one being
   checked.

## Known risks

- The panic hook still prints a backtrace line to stderr for caught panics; harmless but noisy.
- `vehicle.add` leaks its parameter block when it fails.
- `getJunctionFoes` needs `LinkBits` ported before it can return anything.
