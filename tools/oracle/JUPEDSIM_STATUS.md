# JuPedSim models: port status

Files: `crates/sumo_core/src/microsim/transportables/msp_model_ju_ped_sim.rs` (gRPC model) and
`msp_model_ju_ped_sim_legacy.rs` (JuPedSim C API model). Branch `worktree-agent-j01`.

Nothing was compiled (writer-agent rule: no cargo). The lead runs `cargo check -p sumo_core` at merge time.

## Counts (43 `todo!()` per file, 86 total)

| Class | Meaning | `msp_model_ju_ped_sim.rs` | `msp_model_ju_ped_sim_legacy.rs` |
|---|---|---|---|
| (a) pure Rust, ported | faithful translation | 21 | 21 |
| (b) needs GEOS | geos crate, GEOS 3.12.1 in WSL | 14 | 14 |
| (c) needs gRPC/protobuf | tonic + prost, protoc | 7 | 0 |
| (e) needs JuPedSim C API | `jupedsim/jupedsim.h`, bindgen | 0 | 7 |
| (d) needs an unported member or helper | `myActiveLanes`, StopWatch, Command | 1 | 1 |
| **`todo!()` remaining** | | **0** | **0** |
| `shortcut:` + `panic!` stand-ins | | 22 | 22 |

Class (e) is not in the original brief. The legacy model uses the native JuPedSim C library, which
`GEOS_DECISION.md` flags as needing bindgen. No crate was chosen for it.

Totals: 42 ported, 44 documented stand-ins, 0 `todo!()`.

Multi-blocker methods are filed under their first blocker. `drop_` and `initialize` need GEOS first.
`execute` and `new` need gRPC or the C API first and GEOS too.

## Ported (a)

- `MSPModel_JuPedSim`: `clearState`, `removePolygonFromDrawing`, `getRadius`, `polygonChanged`
- `PState`: constructor, destructor (empty, as in C++), `reinit`, `getPosition`, `setPosition`,
  `setAngle`, `setStage`, `setLane`, `setLanePosition`, `getNextEdge`, `getNextWaypoint`,
  `getAgentId`, `setAgentId`, `isFinished`, `getJourneyId`, `getStageId`, `advanceNextWaypoint`
- Legacy: the same set.

## Signature changes

Legacy `PState` accessors were `i32` but the fields are `i64` (C header: `JPS_*` ids are `int64_t`).
Changed to `i64` so they can return the field:
- `get_agent_id(&self) -> i64`
- `set_agent_id(&mut self, id: i64)`
- `get_journey_id(&self) -> i64`
- `get_stage_id(&self) -> i64`

Nothing calls these yet, because the callers are stand-ins.

## Skeleton gaps

- `addWaypoint` and `addWaitingSet` show "no C++ body found" in the skeleton, but the bodies exist:
  `MSPModel_JuPedSim.cpp` lines 202 and 1107 (legacy: lines 201 and 1135). Both are stand-ins.
- `myActiveLanes` (C++ `std::map<MSLane*, std::vector<PState*>>`) is missing from both generated
  structs. `remove` needs it.
- `myNumActivePedestrians` is inherited from `MSPModel_Interacting`, so `clearState` is ported.

## What must be installed or ported to finish

1. **protoc**: `sudo apt-get install -y protobuf-compiler` in WSL. Needed by `tonic-build`.
2. **GEOS**: `sudo apt-get install -y libgeos-dev` (3.12.1 is already in WSL). Then add
   `geos` 11.3.1 to `crates/sumo_core/Cargo.toml`. Unblocks 14 + 14 methods.
3. **gRPC**: add `tonic` 0.14.9, `prost` 0.14.6, and `tonic-build` 0.14.9 (build-dependency).
   Generate the `JuPedSimService` client from the `.proto`. Unblocks 7 methods (ju_ped_sim).
4. **JuPedSim C API**: install `libjupedsim` and bind `jupedsim/jupedsim.h` with bindgen. Unblocks
   7 methods (legacy).
5. **`myActiveLanes`**: add the field to both structs, then port `remove` (legacy: `remove`).
6. **StopWatch**: port `utils/common/StopWatch` and replace `myStopWatch: Vec<i64>`. Only matters
   with `DEBUG_GRPC_TIMING`; the `execute` and `callGrpc` ports depend on it.
7. **Command**: port the C++ `Command` base and replace `Event.base: Unmapped`.
8. **Replace the `Option<Box<dyn Any>>` holders** with `geos::Geometry`, `tonic::transport::Channel`,
   and the generated client. Keep `Vec` and `BTreeMap` shapes.
