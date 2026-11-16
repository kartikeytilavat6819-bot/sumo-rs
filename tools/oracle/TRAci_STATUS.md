# TraCI status (live control path)

Measured 2026-11-16 by `tools/oracle/cases/traci_basic/client.py`. Source of the numbers: `cases/traci_basic/REPORT.md`.
The simulation oracle (`run_all.sh`) does not exercise TraCI at all, so a passing scoreboard says nothing about this path.

## Score

**48 of 62 probes match C++ 1.28.0** (junction 24/31, signal 24/31), measured on branch `worktree-agent-ls04` after
rebasing onto main `88c57f8` with no callsite changes needed. Run: `python tools/oracle/cases/traci_basic/client.py`.
Probes run in separate sessions, so one crash only costs its own row.

First mismatch per failing probe (the same 7 on both cases, except where noted):

| Probe | C++ | Ours |
|---|---|---|
| `vehicle.subscribe(nope)` + step | `TraCIException: Could not add subscription. Vehicle 'nope' is not known.` | `FatalTraCIError: Connection closed by SUMO.` (server dies) |
| `vehicle.changeTarget(v0, last)` + getRoute | junction: `('-103224#0', '-103224#1', '-103224#2', '-103224#3')`; signal: `Vehicle 'v0' is not known.` | `getRouterTT() needs MSRoutingEngine / MSNet router access, which is not ported yet` |
| `vehicle.setRoute(v0, current)` + getRoute | `Route replacement failed for vehicle 'v0' (No connection between edge '-103224#0' and edge '-103224#0'.).` (signal: `'324752521'` → `'324752522'`) | `index out of bounds: the len is 4 but the index is 4` (signal: len 3) |
| `vehicle.add(vnew, route0)` + step + getIDList | junction: `('v0', …, 'v4', 'vnew')`; signal: `('v1', …, 'v4', 'vnew')` | `Vehicle::add not yet ported` |
| `vehicle.add(v0, dup)` | `The vehicle 'v0' to add already exists.` | `Vehicle::add not yet ported` |
| `vehicle.add(vx, nope route)` | `Invalid route 'nope' for vehicle 'vx'.` | `Vehicle::add not yet ported` |
| `vehicle.getJunctionFoes(v0)` | `()` | `Vehicle::getJunctionFoes not yet ported` |

Gate on this branch: `run_all.sh` PASS=25 FAIL=1 SKIP=8 (the FAIL is `reroute/ref_device`); `cargo test -p sumo_core -p utils`
175 passed, 2 ignored; `depart` 15/15.

The `simulation.getTime` segfault and the `getVersion` text are fixed (1e33a68).

## What a Unity client hits today

Starting `sumo.exe --remote-port N`, connecting, and stepping works (`simulationStep` answers `None`,
`getVersion` answers). Then:

- `simulation.getTime`: process **segfaults** (access violation). Every `simulation.*` getter
  that goes through `Simulation::handle_variable` is affected. This is the first thing a client
  will hit, and it kills the server.
- `vehicle.getIDList`, `getPosition`, `getSpeed`, `setSpeed`: `TraCIException: Vehicle::handleVariable not yet ported`.
  The vehicle getter/setter family is a stub. This is the biggest gap for a Unity client.
- `lane.getLastStepVehicleNumber`, `lane.getLength` (valid id), `trafficlight.getIDList`,
  `trafficlight.getRedYellowGreenState`: `TraCIException: pure virtual VariableWrapper::wrapX`.
- `simulation.getLoadedNumber`: 0 where C++ says 3 at t=10.
- Errors for unknown lanes, traffic lights and junctions match C++ text. Errors for unknown vehicles do not
  (they come from the stub).
- `getVersion` answers `SUMO 1.28.0`; C++ answers `SUMO v1.28.0-0000000000` (build-stamp shortcut).

## Root causes and owners

| # | Symptom | Cause | Where | Owner |
|---|---|---|---|---|
| 1 | segfault on `simulation.getTime` | server passes its bare `base2: VariableWrapper` to `Simulation::handle_variable`, which casts it to `Helper_SubscriptionWrapper` | `libsumo/simulation.rs:1231` | ls01 |
| 2 | `pure virtual VariableWrapper::wrapX` | C++ `TraCIServer` overrides the `wrap*` virtuals; the port's `VariableWrapper` is a concrete struct with panic bodies and no dispatch to the server | `libsumo/subscription.rs:136–210`, `libsumo/helper.rs:3529` | ls01 |
| 3 | `Vehicle::handleVariable not yet ported` | stub | `libsumo/vehicle.rs:4558` | ls01 |
| 4 | `getLoadedNumber` 0 vs 3 | not isolated; `BUILT` is informed at `ms_vehicle_control.rs:1724`, the per-step reset is `tra_ci_server.rs:470` | traci_server + ms | unassigned |

Fixing 2 is a design change, not a contained patch: the TraCI server needs to be passed as an object whose
`wrap*` calls reach its own storage. Fixing 1 is the same change.

## Fixed in the TraCI server

- Command panics (`TraCIException` in C++) now answer with an error status and keep the connection
  (`dispatch_command`, `catch_unwind`). Before, one unknown lane killed the process.

## Not covered

`setRedYellowGreenState`, multi-client ordering, `simulation.load`, persons, POIs. Not exercised.
Subscriptions, `changeLane`, `moveToXY`, route setting and add/remove are now probed (table above).
