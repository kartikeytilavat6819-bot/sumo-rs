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

**58 of 62 probes match C++ 1.28.0** (junction 29/31, signal 29/31). Up from 9/32 at the start of the
session, after `VariableWrapper`, `Vehicle::handleVariable`, the wrapper sink, the panic-to-error-status
conversion and a null-RNG fix in `MSVehicleType::computeChosenSpeedDeviation` (C++ allows a null `SumoRNG*`
and means "default generator"; we dereferenced it and panicked, which made `vehicle.add` unusable).

Probes run in separate sessions, so one crash costs only its own row.

## Remaining mismatches (2 per case, the same two)

| Probe | C++ | Ours | Cause |
|---|---|---|---|
| `vehicle.setRoute(v0, current)` + step + getRoute | `TraCIException: Route replacement failed for vehicle 'v0' (No connection between edge …)` | junction: returns the unchanged route instead of failing; signal: `Vehicle 'v0' is not known` | C++ rejects the route when `gCheckRoutes` is set; our `replace_route_edges` (`ms_base_vehicle.rs:1685`) only **warns** and continues. Deliberately not changed: flipping it alters route validation for every simulation, not just TraCI |
| `vehicle.getJunctionFoes(v0)` | `()` | `TraCIException: Vehicle::getJunctionFoes not yet ported` | `LinkBits` is `()` in `ms_logic_junction.rs` (the same `Unmapped` class `VariableWrapper` was) |

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
