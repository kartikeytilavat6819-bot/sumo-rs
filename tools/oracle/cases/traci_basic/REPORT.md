# traci_basic — REPORT

Run: `python tools/oracle/cases/traci_basic/client.py` (C++ 1.28.0 in WSL vs our `sumo.exe` built from
branch `worktree-agent-tc01`, private target dir `sumo-rs-target-tc01`, because the shared
`debug/sumo.exe` was being overwritten by other worktrees during this run). Client: Python `traci` 1.28.0.
Each probe runs in a fresh server session after two `simulationStep` calls.

**Result: 9 of 32 probes match** (junction 5/16, signal 4/16).

## Junction (`junction/ref_simple.net.xml`, lane `-103224#1_0`, no traffic light)

| command | C++ | ours | verdict |
|---|---|---|---|
| getVersion | `(22, 'SUMO v1.28.0-0000000000')` | `(22, 'SUMO 1.28.0')` | DIFF (version string shortcut) |
| simulationStep(11) | None | None | MATCH |
| simulation.getTime | `10.0` | connection closed: **segfault** | DIFF |
| simulation.getLoadedNumber | `3` | `0` | DIFF |
| simulation.getArrivedNumber | `0` | `0` | MATCH |
| vehicle.getIDList | 5 ids | `TraCIException: Vehicle::handleVariable not yet ported` | DIFF |
| vehicle.getPosition(v0) | `(140.726…, 147.925…)` | same stub | DIFF |
| vehicle.getSpeed(v0) | `6.637…` | same stub | DIFF |
| lane.getLastStepVehicleNumber | `1` | `pure virtual VariableWrapper::wrapInt` | DIFF |
| lane.getLength | `60.12` | `pure virtual VariableWrapper::wrapDouble` | DIFF |
| trafficlight.getIDList | `()` | `pure virtual VariableWrapper::wrapStringList` | DIFF |
| trafficlight.getRedYellowGreenState('none') | `TraCIException: Traffic light 'none' is not known` | same | MATCH |
| setSpeed(v0,5)+step+getSpeed | `5.0` | `Vehicle::handleVariable not yet ported` | DIFF |
| vehicle.getSpeed('nope') | `Vehicle 'nope' is not known.` | `Vehicle::handleVariable not yet ported` | DIFF (wrong error text) |
| lane.getLength('nope') | `Lane 'nope' is not known` | same | MATCH |
| trafficlight.getRedYellowGreenState('nope') | `Traffic light 'nope' is not known` | same | MATCH |

## Signal (`signal/ref_simple.net.xml`, lane `-324752522_0`, TL `1802245009`)

| command | C++ | ours | verdict |
|---|---|---|---|
| getVersion | `(22, 'SUMO v1.28.0-0000000000')` | `(22, 'SUMO 1.28.0')` | DIFF |
| simulationStep(11) | None | None | MATCH |
| simulation.getTime | `10.0` | segfault | DIFF |
| simulation.getLoadedNumber | `3` | `0` | DIFF |
| simulation.getArrivedNumber | `0` | `0` | MATCH |
| vehicle.getIDList / getPosition / getSpeed | tuples / float | `Vehicle::handleVariable not yet ported` | DIFF |
| lane.getLastStepVehicleNumber / lane.getLength | `1` / `33.53` | `pure virtual VariableWrapper` | DIFF |
| trafficlight.getIDList | `('1802245009',)` | `pure virtual VariableWrapper::wrapStringList` | DIFF |
| trafficlight.getRedYellowGreenState | `'GGGGg'` | `pure virtual VariableWrapper::wrapString` | DIFF |
| setSpeed+step+getSpeed | `6.015…` | stub | DIFF |
| vehicle.getSpeed('nope') | `Vehicle 'nope' is not known.` | stub | DIFF |
| lane.getLength('nope') | `Lane 'nope' is not known` | same | MATCH |
| trafficlight.getRedYellowGreenState('nope') | `Traffic light 'nope' is not known` | same | MATCH |

## What the mismatches are

1. **Segfault on `simulation.getTime`** (and every simulation/vehicle getter that goes through
   `Simulation::handle_variable`). `TraCIServer` passes its bare `base2: VariableWrapper` to
   `libsumo::Simulation::handle_variable`, which casts it to `Helper_SubscriptionWrapper`. The cast is
   wrong, so the process dies with access violation `0xC0000005`. Not catchable with `catch_unwind`.
   Owner: `libsumo/simulation.rs` (ls01). Fix: pass the server as a real `VariableWrapper` with virtual dispatch.
2. **`pure virtual VariableWrapper::wrapX` for lane and traffic-light getters.** In C++ `TraCIServer`
   overrides `wrapInt/wrapDouble/wrapString/wrapStringList` (`TraCIServer.h:59`). In the port,
   `libsumo::VariableWrapper` is a concrete struct whose `wrap*` bodies are `panic!`, and the
   server's own `wrap_*` methods (`tra_ci_server.rs:1085`) are never reached. Owner: `libsumo/subscription.rs` and `libsumo/helper.rs`.
3. **`Vehicle::handleVariable not yet ported`** (`libsumo/vehicle.rs:4558`). The whole vehicle getter/setter
   family is a stub. Owner: ls01.
4. **`getLoadedNumber` 0 vs 3.** `BUILT` is informed in `ms_vehicle_control.rs:1724`, and the server's
   listener is registered in `open_socket`. The count is still 0 at step 10, so the per-step state
   read/reset in `process_commands` (`tra_ci_server.rs:470`) or listener wiring is suspect. Not isolated.
5. **`getVersion` text.** C++ uses the generated `VERSION_STRING` (`v1.28.0-0000000000`), ours uses
   the fallback in the existing shortcut (`tra_ci_server.rs:49`).
6. **Error text for `vehicle.getSpeed('nope')`** comes from the stub, not the C++ message.

## Fixed in this branch

- `dispatch_command` now wraps executor and getter calls in `catch_unwind` and answers with
  `writeErrorStatusCmd`, like C++'s `catch (libsumo::TraCIException&)`. Before, any libsumo error killed
  the whole `sumo.exe` (exit 1 after "Quitting (on error)"). After: `lane.getLength('nope')` returns
  the C++ `TraCIException` text and the connection stays up. Verified with `probe_err`.

## Not covered by this case

Subscriptions, `changeLane`, `setRedYellowGreenState`, multi-client ordering, `simulation.load`,
and anything not listed in `client.py`.
