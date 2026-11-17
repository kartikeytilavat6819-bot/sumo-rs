# UB audit: writes through `&self` (`Ptr::new(self).m()` and `*const Self as *mut Self`)

Rule: a `&self` method must not write a field that a `&self` method reads. The optimiser
assumes it cannot happen, so it can return stale values.

Verdicts:
- **fixed**: `Cell` for scalars (`.get()` / `.set()`), `RefCell` for collections (`.borrow()` / `.borrow_mut()`). Returned references become owned clones where a `RefCell` cannot lend one.
- **deferred**: real UB, not fixed. The reason is given.
- **legit**: no `&self` method reads the written field, or the method takes `&mut self`.

Line numbers are from the pre-audit HEAD (`a4e7db0`).

## Status

| Commit | Batch | Sites |
|---|---|---|
| `47bb353` (merged `12ee4fb`) | scalar `Cell`s | `ms_edge` 2551, 3289; `ms_lane` 2574, 2652 (times); `msp_model_striping` 2867; `nb_traffic_light_definition` 901, 936; `nb_own_tl_def` 1932; `nb_loaded_sumotl_def` 1334-1335; `cch_graph` 364, 415 |
| `d72636c` | `NBEdge` successor caches | `nb_edge` 8114-8175 (`get_successors`, `get_via_successors`) |
| `c5f0652` | railway `svc*` caches | `nb_algorithms_railway` 3116, 3138 (both `*const Self as *mut Self` casts removed) |
| `0723062` | `MSEdge` waiting list and failed-insertion memory, CCH arc permissions | `ms_edge` 2109, 2116, 3614-3617, 3646, 3685; `cch_graph` 323, 363, 411 |
| `923d923` | traffic-light need-cont-relation and extra-conflict sets | `nb_traffic_light_definition` 896, 932; `nb_own_tl_def` 1920, 1928; `nb_loaded_tl_def` 459, 487; `nb_loaded_sumotl_def` 1275-1276, 1316, 1326 |

## Deferred

| Site | Why |
|---|---|
| `microsim/ms_edge.rs` 3247, 3249 (`add_transportable`), 3259-3261 (`remove_transportable`) | `myPersons` / `myContainers` need `RefCell`, which changes `get_persons()` to return `Ref`. That ripples into callers in `ms_link.rs` (3807, 3814), `ms_lane.rs` (3908, 8086, 9010), `ms_vehicle.rs` (13562, 13574), and `libsumo`/output files. Those are owned by other agents, so this is a separate step to revert alone. |
| `microsim/ms_lane.rs` 2573, 2651 (`MSLeaderInfo` leader/follower caches) | `ms_lane.rs` is owned by another agent (MSGlobals sweep and arrival-speed work). The fix is `RefCell<MSLeaderInfo>`; it is a few lines and ready to apply once the file is free. |
| `microsim/ms_lane.rs` `let this = Ptr::new(self)` alias sites (8227, 8546, 8694, 8915) | Not audited. `ms_lane.rs` is owned by another agent. |
| `microsim/ms_vehicle.rs` `let me = Ptr::new(self)` sites (8644, 13388, 14539, 14772, 14896, 15283, 15424, 15498) | Not audited. `ms_vehicle.rs` is owned by another agent. |
| `microsim/ms_base_vehicle.rs`, `microsim/cfmodels/mscf_model*`, `microsim/output/ms_mean_data.rs`, `microsim/ms_link.rs` | Not audited. Owned by other agents this round. |
| `netbuild/nb_node.rs` 2283, 2585; `netgen/ng_node.rs` 277 (`let me/this = Ptr::new(self)`) | Not audited: needs a check of each method's receiver and writes. |
| `router/ro_route_def.rs` 235, 501 (`Ptr::new(self).get_mut()`) | Not audited. Needs a check whether the method is `&self`. |
| `microsim/output/mse3_collector.rs` 204, 209 | Not audited. |

## Legit (no `&self` write to a read field)

| Site | Why |
|---|---|
| `microsim/ms_edge.rs` 4023 `invalidate_meso_cache`: `myLastCacheUpdate` | Never read in the crate (only in commented C++). Caveat below. |
| `netbuild/nb_loaded_sumotl_def.rs` 1883: `Ptr::new(self).m().get_max_index()` | No field write. `get_max_index` takes `&mut self`. Caveat below. |
| `microsim/devices/ms_device_vehroutes.rs` 421-437 `notify_enter` | Takes `&mut self`. |
| `netbuild/nb_edge.rs` 4107 `compute_edge2_edges` | Takes `&mut self`. |

Comment-only (no live code): `utils/router/car_edge.rs` 96, 137; `flipped_node.rs` 78, 95; `flipped_edge.rs` 151; `reversed_edge.rs` 176; `rail_edge.rs` 394. These will need `Cell`/`RefCell` when those routers are ported.

## Caveats

- Under the Rust aliasing model, a write through `&self` to a field without `UnsafeCell` is UB even when nothing reads it. The two legit `&self` rows (`myLastCacheUpdate`, `get_max_index`) are not fully safe. Converting `myLastCacheUpdate` to `Cell` is one line. `get_max_index` needs `&self` in its trait, which changes a virtual signature.
- The junction, roundabout, and real cases don't exercise these caches in a way that would show stale values, so identical output shows no regression, not that the UB is gone.

## Verification (batch 4, `923d923`, and the same for every batch above)

- Cases: `junction`, `roundabout`, `real`. Release and debug run with the same command line as `run_all.sh` (including `sim.args`). `tripinfo.xml`, `fcd.xml`, and exit code compared after dropping the `generated on` stamp and output-path lines.
- Release == debug on all three cases for every batch. Output identical to the pre-audit baseline on all three.
- Scoreboard (`run_all.sh` logic, run from a copy because `run_all.sh` hardcodes the main checkout and `sumo-rs-target-main`): PASS=8 FAIL=2 SKIP=0 before and after every batch. The two FAILs (`real_dense` sim, `signal` sim) fail on the baseline too. No PASS row lost.
- Baseline is the current `wave2` (`12ee4fb`), which includes the junction fix `0868319`. `main` does not contain the UB commits; `wave2` is `main` plus them.

## Round 2 (ub02): whole-crate sweep, `&self` writes and const-casts

Sweep: `Ptr::new(self)` / `get_mut()` / `as_raw()` aliases, `*const as *mut` casts, and `.m().field =` writes through aliases of `self`. Each C++ `mutable` member that was checked is listed.

| Site | C++ | Rust fix | Verdict |
|---|---|---|---|
| `utils/common/stable_cache.rs` (new) | `mutable std::map` filled by const methods, refs returned | boxed values behind `RefCell`, refs stay valid (`get_or_insert_with`, `clear`, `clone`) | new primitive |
| `utils/router/car_edge.rs` successors / via-successors | `mutable myClassesSuccessorMap`, `myClassesViaSuccessorMap` | `StableCache` (was `Ptr::get_mut` on `&self`) | fixed |
| `utils/router/reversed_edge.rs`, `rail_edge.rs`, `flipped_edge.rs` via-successors | `mutable myClassesViaSuccessorMap` | `StableCache`; `&mut *as_raw()` casts removed | fixed |
| `utils/router/flipped_node.rs` incoming / outgoing | `mutable myIncoming`, `myOutgoing` | `OnceCell` (lends `&T`) | fixed |
| `microsim/ms_edge.rs` successors / via-successors | `mutable myClassesSuccessorMap`, `myClassesViaSuccessorMap`, `myOrigClassesViaSuccessorMap` | `StableCache` | fixed |
| `microsim/ms_edge.rs` reversed / railway routing edge | `mutable myReversedRoutingEdge`, `myRailwayRoutingEdge` | `Cell<Ptr<..>>` | fixed |
| `microsim/ms_edge.rs` `myPersons`, `myContainers` (`add_transportable` const) | `mutable std::set` | `RefCell`; `get_persons()` returns `Ref` (only `ms_induct_loop` changed to `.iter()`) | fixed (was deferred) |
| `microsim/ms_edge.rs` `myCachedMesoPos`, `myLastCacheUpdate` (`get_meso_positions`, `invalidate_meso_cache`) | `mutable` | `RefCell<Rc<map>>` (returns an `Rc` clone), `Cell<SUMOTime>` | fixed (`myLastCacheUpdate` was wrongly marked legit: it is read) |
| `microsim/ms_lane.rs` `myLeaderInfo`, `myFollowerInfo` | `mutable MSLeaderInfo` | `RefCell<MSLeaderInfo>` | fixed (was deferred) |
| `microsim/ms_route.rs` `mySavings` (`set_savings` via const_cast in `ms_triggered_rerouter.rs`) | written through `&MSRoute` | `Cell<f64>`, `set_savings(&self)` | fixed |
| `netbuild/nb_node.rs` `myDisplacementError` (`compute_smooth_shape(&self)` writes `recordError`) | non-`mutable` field, read by `get_displacement_error(&self)` | `Cell<f64>` | fixed |
| `router/ro_edge.rs` successors / via-successors | `mutable` | `StableCache`; `as_mut_edge` helper removed | fixed |
| `router/ro_edge.rs` reversed / flipped / railway routing edge | `mutable` | `Cell<Ptr<..>>` | fixed |
| `router/ro_route_def.rs` `myPrecomputed`, `myNewRoute`, `myDiscardSilent` | `mutable` | `RefCell<Option<Rc>>`, `Cell<bool>` x2 | fixed (was deferred) |
| `router/ro_node.rs` `myFlippedRoutingNode` | `mutable` | `Cell<Ptr<..>>` | fixed |

Deferred (real UB, not fixed in this round):

| Site | Why |
|---|---|
| `libsumo/person.rs` 596 (`set_parameter`), `microsim/devices/ms_device_taxi.rs` 264 (`line`), `microsim/ms_vehicle.rs` 1639 (`stops.clear()`) | Write `SUMOVehicleParameter` through a `*mut` cast of a shared reference. C++ members `mutable line`, `stops`, `via`, `color`. Fix is `RefCell` for `stops`/`line`/`via`, but about 25 readers in `ro_*`, `ms_*`, `libsumo` need `.borrow()`; owned by other agents. |

Legit (checked, no `&self` write to a field that a `&self` method reads):

| Site | Why |
|---|---|
| `mse2_collector.rs` 195, `mse3_collector.rs` 161, `ms_induct_loop.rs` 121, `ms_instant_induct_loop.rs` 68 | `*const as *mut` only to call a `&self` method (read-only). |
| `ms_link.rs` 3331, 3352 `get_corresponding_*_link` | read-only traversal via `Ptr::new(self)`. |
| `nb_node.rs` `self_ptr` casts (3039, 7299, 7498, 7996, 8238) | no `.m()` writes through them. |
| `nb_edge.rs` 4076 alias | enclosing fn takes `&mut self`. |
| `ms_link.rs` 4439 | unit test, no concurrent borrow. |
| `cfmodels/*` `vars.m().x = ...` | write a `VehicleVariables` heap object reached through a raw pointer stored in the vehicle, not inside the `&self` referent. |

Not yet audited: `let me/this = Ptr::new(self)` aliases in `ms_lane.rs` (8227, 8546, 8694, 8915), `ms_vehicle.rs` (8644, 13388, 14539, 14772, 14896, 15283, 15424, 15498), `ms_drive_way.rs` (1097, 4567), `ms_rail_signal.rs` (977-1199), `dfrouter/rodf_net.rs` (311, 2137), `mesosim/me_segment.rs` (747), `ms_base_vehicle.rs`, `ms_mean_data.rs`, `ms_link.rs`. Only the `.m().field =` / `.push` / `.insert` patterns were grepped for these, and none were hit except the ones fixed above.

Verification (round 2): release run (`PROFILE=release bash tools/oracle/run_all.sh`, `--release` build in `$CARGO_TARGET_DIR-rel`) gave the same verdict on every row as the debug gate for commit `c8cad490` (PASS=214 FAIL=4 SKIP=14; the same four known rows: bluelight, calibrator_multi, peds_crossing, routeprobe). Later commits are verified by the final gate.
