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

## Round 3 (ub03): the round-2 leftovers, and the "not yet audited" aliases

Closes the round-2 deferred row (`SUMOVehicleParameter` writes) and the "Not yet audited" list above. Every `Ptr::new(self)` / `from_mut` / `as *mut` alias in `ms_lane`, `ms_vehicle`, `ms_drive_way`, `ms_rail_signal`, `rodf_net`, `me_segment`, `ms_edge`, `ms_link`, `nb_node`, `nb_loaded_sumotl_def`, `ms_net`, `ms_parking_area`, `ms_charging_station`, `ms_vehicle_type`, `ms_base_vehicle` was read for writes (`.m().field =`, `alias.field =`, `alias.m().mut_method()`, `DerefMut` calls).

| Site | C++ | Rust fix | Verdict |
|---|---|---|---|
| `libsumo/person.rs` `set_parameter`, `set_color`; `microsim/devices/ms_device_taxi.rs` `build_vehicle_devices`; `microsim/ms_vehicle.rs` remote-control route update (stops.clear) | `mutable line, stops, via, color, parametersSet` on `SUMOVehicleParameter` (const_cast callers) | owner-based `&mut`: `MSBaseVehicle` / `MSTransportable::get_parameter_mutable()` (returns `myParameter.m()`), `SUMOVehicle` vtable `get_parameter_mutable`, MSVehicle and MEVehicle entries | fixed (was deferred) |
| `microsim/ms_vehicle.rs` `plan_move_internal` (`&self`), `me.m().myStops[..].waypointWithStop = true` | `const_cast<MSStop&>(stop).waypointWithStop` in a `const` method | `MSStop::waypointWithStop: Cell<bool>`; reader in `ms_stop_out.rs` uses `.get()` | fixed |
| `microsim/ms_vehicle.rs` 4563, 4660 (dead `veh` casts), 4867 | none (dead / read-only) | casts removed (`Ptr::new(self)`) | fixed |
| `microsim/traffic_lights/ms_rail_signal.rs` `get_blocking_vehicle_i_ds`, `get_rival_vehicle_i_ds`, `get_priority_vehicle_i_ds`, `get_constraint_info_2`, `get_blocking_drive_way_i_ds`, `get_requested_drive_way_2` | `const` methods doing `const_cast<MSRailSignal*>(this)` to call non-const getters | `&mut self` + `Ptr::from_mut(self)`; no callers in the crate | fixed |
| `dfrouter/rodf_net.rs` `compute_types`, `build_routes`, `build_detector_edge_dependencies` | `mutable myDetectorsOnEdges`, `myDetectorEdges` (rebuilt by const methods) | `&mut self`; `dfrouter_main.rs` calls through `optNet.m()` | fixed (signature change, listed) |
| `mesosim/me_segment.rs` `get_mean_speed(&self)` | `mutable myMeanSpeed`, `myLastMeanSpeedUpdate` | `Cell<f64>`, `Cell<SUMOTime>` | fixed |
| `router/ro_route.rs` `myCosts`, `myProbability`; written via `Rc<RORoute>` in `ro_route_def.rs` (5 sites: `Ptr::new(&*alt).get_mut()`) | plain members written through `RORoute*` in `RORouteDef` | `Cell<f64>` for both; `set_costs` / `set_probability` take `&self`; `ro_route_def` calls them through the `Rc` | fixed |
| `microsim/output/ms_induct_loop.rs` `myNextMesoLeaveTimes`, `myLastMesoLeaveTimes` (written in `get_last_detection_time(&self)`) | `mutable TimeQueue` | `RefCell<TimeQueue>`; `get_last_detection_time` keeps `&self` (its callers are `&self` and in `ms_actuated`) | fixed |
| `netbuild/nb_loaded_sumotl_def.rs` `init_needs_cont_relation` (`me.m().base.init_*`) | calls `const` base methods | calls `self.base.*` directly (no `&mut` through an alias) | fixed (cleanup) |
| `microsim/ms_base_vehicle.rs` `get_prefixed_parameter` (`microVeh.m().get_lane_change_model()`) | `getLaneChangeModel() const` overload | `get_lane_change_model_2()` (`&self`) | fixed |
| `microsim/devices/ms_device_fcd_replay.rs` `found = Ptr::from_raw(t as *const _ as *mut _)` | none (pointer copy) | `Ptr::new(t)` | fixed |
| `microsim/ms_lane.rs` (all `Ptr::new(self)` aliases), `ms_drive_way.rs`, `ms_link.rs`, `ms_edge.rs`, `nb_node.rs` (`me`/`self_ptr`), `ms_parking_area.rs` (`me = Ptr::new(forVehicle)`) | | audited: aliases are only passed as identity or read; the `let mut` aliases are only reassigned | legit |
| `microsim/ms_net.rs` `get_router_tt` / `get_intermodal_router` / `reset_intermodal_router` | `mutable std::map` | already `RefCell` (earlier batch); `get_router_tt` returns `&mut` to a heap router owned through a `Ptr` (not inline in `MSNet`) | legit |
| `microsim/ms_vehicle_type.rs` `duplicate_type` (`vtype.m().myOriginalType = Ptr::new(self)`) | none | writes the new object, not `self` | legit |
| `microsim/output/mse2_collector.rs`, `mse3_collector.rs`, `ms_induct_loop.rs`, `ms_instant_induct_loop.rs`, `trigger/ms_calibrator.rs` `*mut` helper casts | none | the helper only dispatches to `&self` `write_xml_detector_prolog` | legit (cast kept) |
| `microsim/ms_base_vehicle.rs` `as_micro_vehicle` (`self as *mut MSVehicle`) | `dynamic_cast<MSVehicle*>` | callers only read | legit |

Deferred (real UB, not fixed in this round):

| Site | Why |
|---|---|
| `microsim/devices/ms_device_transportable.rs` 715-719 `notify_move_internal` (`Ptr::from_raw(veh as *const _ as *mut _)`, then `notify_move(vp.m(), ...)`) | C++ `const_cast<SUMOTrafficObject&>(veh)`. Fixing it needs `&mut SUMOTrafficObject` through `MSMoveReminder::notify_move_internal` and every device/detector override that forwards it (about 15 files). Those files belong to other batches. |

Verification (round 3): `tools/kaggle/agent_run.sh` builds `--release` into `$CARGO_TARGET_DIR-rel`, runs `PROFILE=debug` and `PROFILE=release bash tools/oracle/run_all.sh`, and diffs the verdict columns. Result on `f069bb8f`: debug and release both PASS=301 FAIL=6 SKIP=41, 0 differing verdict rows (the six known FAILs are the same in both: containers, rerouter_parking, taxi x2, tls_waut, busride). Depart 15/15, unit tests pass.

## Base-offset casts (multiple inheritance, non-first base fields)

A struct whose C++ class has a second base is ported with that base in a later field (`base2`, `base3`, `base4`). A pointer to that base is at `&s.baseN`, not at `s`. Converting it to the derived type (or back) needs `offset_of!`. `.cast::<T>()` is only correct for the first base.

Fixed in this round:

| Site | C++ | Problem | Fix |
|---|---|---|---|
| `libsumo/lane_area.rs` `get_detector` | `static_cast<MSE2Collector*>(MSDetectorFileOutput*)`; `MSE2Collector : MSMoveReminder, MSDetectorFileOutput` | `d.cast::<MSE2Collector>()` read the detector at offset 0 (the MSMoveReminder), so the pointer was wrong | `mse2_collector::mse2_collector_of(d)` (offset of `base2`, mirrors `induct_loop_of`) |
| `traci_server/tra_ci_server.rs` `TraCIServer::initialise` | `addTransportableStateListener(this)`: `TraCIServer : VehicleStateListener, VariableWrapper, TransportableStateListener` | `inst.cast::<MSNet_TransportableStateListener>()` pointed at offset 0 (`base`, the vehicle listener) | `Ptr::from_mut(&mut inst.m().base3)` |

Verified correct (no change): every other `.cast::<X>()` whose target is a derived type with the base at offset 0. Specific checks:

- `MSInductLoop` (`base2` MSDetectorFileOutput): `induct_loop_of`, `detector_of`, `ms_induct_loop_of`, and the vtable helpers all apply the offset.
- `MSInstantInductLoop` (`base2`), `MSRouteProbe` (`base2` MSMoveReminder), `MSTriggeredRerouter` (`base2` MSMoveReminder, `base4` MSStoppingPlaceRerouter), `MSCalibrator` (`base3` MSDetectorFileOutput), `MSE2Collector` (`base2`, helper `mse2_of`), `MSE3Collector` (MSDetectorFileOutput first), `MSDelayBased`/`NEMA` detector casts: offsets applied.
- `MSVehicleDevice` (`base` MSMoveReminder, `base2` MSDevice): all `Ptr<MSDevice>` upcasts use `&dev.base2` (`me_vehicle`, `ms_vehicle`). `Ptr<MSVehicleDevice>` → `MSDevice_GLOSA` / `StationFinder` / `FCD` / `Taxi` / `Tripinfo` / `ToC` / `Bluelight` / `Transportable` / `VehRoutes` all use derived types that have `MSVehicleDevice` first, so the cast is at offset 0.
- `MSTransportableDevice` (`base2` MSMoveReminder): no derived-pointer casts from a move reminder in the crate.
- `MSDriveWay` (`base2` Named): MR is first, so the `cast::<MSDriveWay>()` / `cast::<MSMoveReminder>()` sites are fine. No `Named` cast to a drive way.
- `MSDetectorFileOutput` (`base2` Parameterised): no Parameterised↔detector casts in the crate. `MSMeanData` has the detector base first.
- `PointOfInterest` (`base` Shape, `base2` Position, `base3` Parameterised) and `SUMOPolygon` (`base2` Parameterised): `cast::<Named>()` is fine (Shape → Named). No Parameterised casts from these.
- `MSEdge` (`base2` Parameterised), `MSLane`, `MSJunction`, `MSRoute`, `MSStoppingPlace`, `MSTrafficLightLogic`, `NBEdge`, `NBNode`, `ROEdge`: Named is first, so the `cast::<Named>()` sites are fine.
- `MSSOTL*` policies (`PushButtonLogic` / `SigmoidLogic` as `base2`/`base3`), `MSDevice_FCDReplay` / `MSLaneSpeedTrigger` / `DataHandler` (`base2` GenericSAXHandler or MSMapMatcher), `MSRouteHandler` / `RORouteHandler` (`base2` MapMatcher): no casts into or out of those non-first bases.
- `ms_device_transportable.rs` `notify_move_internal` (`const_cast` of `veh`): not an offset issue, still listed under the "Deferred" table above.

Known gap, not an offset bug: `MSNet_TransportableStateListener` is an empty struct with no vtable. `MSNet::transportable_state_changed` calls its no-op method, so the TraCIServer override is never reached. The pointer is now correct, and behavior is unchanged. Upgrade with a vtable for the listener when TraCI transportable state reporting is ported.
